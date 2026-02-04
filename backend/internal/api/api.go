package api

import (
	"fmt"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"

	"clickploy/internal/builder"
	"clickploy/internal/deployer"
	"clickploy/internal/ports"
)

type Handler struct {
	builder  *builder.Builder
	deployer *deployer.Deployer
	ports    *ports.Manager
}

func NewHandler(b *builder.Builder, d *deployer.Deployer, p *ports.Manager) *Handler {
	return &Handler{
		builder:  b,
		deployer: d,
		ports:    p,
	}
}

type DeployRequest struct {
	Repo     string `json:"repo" binding:"required"`
	Name     string `json:"name" binding:"required"`
	Port     int    `json:"port"`
	GitToken string `json:"git_token"`
}

type DeployResponse struct {
	Status  string `json:"status"`
	AppName string `json:"app_name"`
	Port    int    `json:"port"`
	URL     string `json:"url"`
	Message string `json:"message"`
}

func (h *Handler) RegisterRoutes(r *gin.Engine) {
	r.POST("/deploy", h.handleDeploy)
	h.RegisterStreamRoutes(r)
}

func (h *Handler) handleDeploy(c *gin.Context) {
	var req DeployRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	imageName, err := h.builder.Build(req.Repo, req.Name, req.GitToken, "", "", "", "", nil, os.Stdout)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Build failed: %v", err)})
		return
	}

	port, err := h.ports.GetPort(req.Name, req.Port)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Port allocation failed: %v", err)})
		return
	}

	_, err = h.deployer.RunContainer(c.Request.Context(), imageName, req.Name, port, nil)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Deployment failed: %v", err)})
		return
	}

	c.JSON(http.StatusOK, DeployResponse{
		Status:  "success",
		AppName: req.Name,
		Port:    port,
		URL:     fmt.Sprintf("http://localhost:%d", port),
		Message: "Container started successfully",
	})
}

func (h *Handler) RegisterSystemRoutes(r *gin.Engine) {
	r.GET("/api/system/status", h.handleSystemStatus)
}

func (h *Handler) handleSystemStatus(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"version": "v0.1.0",
		"status":  "All systems normal",
	})
}
