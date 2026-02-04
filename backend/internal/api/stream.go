package api
import (
	"net/http"
	"sync"
	"time"
	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
)
var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true },
}
type LogHub struct {
	mu      sync.Mutex
	streams map[string][]chan []byte
}
var Hub = &LogHub{
	streams: make(map[string][]chan []byte),
}
func (h *LogHub) Broadcast(deploymentID string, p []byte) {
	h.mu.Lock()
	defer h.mu.Unlock()
	if sinks, ok := h.streams[deploymentID]; ok {
		for _, sink := range sinks {
			select {
			case sink <- p:
			default:
			}
		}
	}
}
func (h *LogHub) Subscribe(deploymentID string) chan []byte {
	h.mu.Lock()
	defer h.mu.Unlock()
	ch := make(chan []byte, 256)
	h.streams[deploymentID] = append(h.streams[deploymentID], ch)
	return ch
}
func (h *LogHub) Unsubscribe(deploymentID string, ch chan []byte) {
	h.mu.Lock()
	defer h.mu.Unlock()
	if sinks, ok := h.streams[deploymentID]; ok {
		for i, sink := range sinks {
			if sink == ch {
				h.streams[deploymentID] = append(sinks[:i], sinks[i+1:]...)
				close(ch)
				break
			}
		}
		if len(h.streams[deploymentID]) == 0 {
			delete(h.streams, deploymentID)
		}
	}
}
type StreamWriter struct {
	DeploymentID string
}
func (w *StreamWriter) Write(p []byte) (n int, err error) {
	c := make([]byte, len(p))
	copy(c, p)
	Hub.Broadcast(w.DeploymentID, c)
	return len(p), nil
}
func (h *Handler) streamDeploymentLogs(c *gin.Context) {
	deploymentID := c.Param("id")
	if deploymentID == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid ID"})
		return
	}
	conn, err := upgrader.Upgrade(c.Writer, c.Request, nil)
	if err != nil {
		return
	}
	defer conn.Close()
	logChan := Hub.Subscribe(deploymentID)
	defer Hub.Unsubscribe(deploymentID, logChan)
	go func() {
		for {
			if _, _, err := conn.NextReader(); err != nil {
				conn.Close()
				return
			}
		}
	}()
	for logChunk := range logChan {
		conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
		if err := conn.WriteMessage(websocket.TextMessage, logChunk); err != nil {
			return
		}
	}
}
func (h *Handler) RegisterStreamRoutes(r *gin.Engine) {
	r.GET("/api/deployments/:id/logs/stream", h.streamDeploymentLogs)
}
