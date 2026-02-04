package ports
import (
	"fmt"
	"math/rand"
	"net"
	"sync"
)
type Manager struct {
	mu          sync.Mutex
	startPort   int
	endPort     int
	allocations map[string]int
}
func NewManager(start, end int) *Manager {
	return &Manager{
		startPort:   start,
		endPort:     end,
		allocations: make(map[string]int),
	}
}
func (m *Manager) GetPort(appName string, specificPort int) (int, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if port, exists := m.allocations[appName]; exists {
		if specificPort > 0 && specificPort != port {
			return 0, fmt.Errorf("app %s is already running on port %d", appName, port)
		}
		return port, nil
	}
	if specificPort > 0 {
		if err := m.checkPortAvailable(specificPort); err != nil {
			return 0, err
		}
		m.allocations[appName] = specificPort
		return specificPort, nil
	}
	rangeSize := m.endPort - m.startPort + 1
	offsets := rand.Perm(rangeSize)
	for _, offset := range offsets {
		port := m.startPort + offset
		if err := m.checkPortAvailable(port); err == nil {
			if !m.isPortAllocatedInternal(port) {
				m.allocations[appName] = port
				return port, nil
			}
		}
	}
	return 0, fmt.Errorf("no available ports in range %d-%d", m.startPort, m.endPort)
}
func (m *Manager) isPortAllocatedInternal(port int) bool {
	for _, p := range m.allocations {
		if p == port {
			return true
		}
	}
	return false
}
func (m *Manager) checkPortAvailable(port int) error {
	if port < m.startPort || port > m.endPort {
		return fmt.Errorf("port %d is out of allowed range %d-%d", port, m.startPort, m.endPort)
	}
	if m.isPortAllocatedInternal(port) {
		return fmt.Errorf("port %d is internally allocated", port)
	}
	addr := fmt.Sprintf("localhost:%d", port)
	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return nil
	}
	_ = conn.Close()
	return fmt.Errorf("port %d is already in use", port)
}
