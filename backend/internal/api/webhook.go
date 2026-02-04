package api

import (
	"bytes"
	"clickploy/internal/db"
	"clickploy/internal/models"
	"fmt"
	"io"
	"net/http"

	"github.com/gin-gonic/gin"
	gonanoid "github.com/matoous/go-nanoid/v2"
)

func (h *Handler) RegisterWebhookRoutes(r *gin.Engine) {
	r.POST("/projects/:projectID/webhook/:webhookID", h.handleWebhook)
}
func (h *Handler) handleWebhook(c *gin.Context) {
	projectID := c.Param("projectID")
	webhookSecret := c.Param("webhookID")
	if projectID == "" || webhookSecret == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid webhook url"})
		return
	}
	var project models.Project
	if result := db.DB.Preload("EnvVars").Where("id = ?", projectID).First(&project); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}
	if project.WebhookSecret != webhookSecret {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid Webhook Secret"})
		return
	}
	depID, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 10)
	deployment := models.Deployment{
		ID:        depID,
		ProjectID: project.ID,
		Status:    "building",
		Commit:    "WEBHOOK",
		Logs:      "Webhook triggered. Starting build...",
	}
	if err := db.DB.Create(&deployment).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create deployment record"})
		return
	}
	go func() {
		var logBuffer bytes.Buffer
		streamer := &StreamWriter{DeploymentID: deployment.ID}
		multi := io.MultiWriter(&logBuffer, streamer)
		envMap := make(map[string]string)
		for _, env := range project.EnvVars {
			envMap[env.Key] = env.Value
		}
		imageName, commitHash, err := h.builder.Build(project.RepoURL, "", project.Name, project.GitToken, project.BuildCommand, project.StartCommand, project.InstallCommand, project.Runtime, envMap, multi)
		deployment.Logs = logBuffer.String()
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nBuild Failed: %v", err)
			db.DB.Save(&deployment)
			return
		}
		if commitHash != "" {
			deployment.Commit = commitHash
		}
		var envStrings []string
		for _, env := range project.EnvVars {
			envStrings = append(envStrings, fmt.Sprintf("%s=%s", env.Key, env.Value))
		}
		containerID, err := h.deployer.RunContainer(c.Request.Context(), imageName, project.Name, project.Port, envStrings)
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nDeployment Failed: %v", err)
			db.DB.Save(&deployment)
			return
		}
		deployment.Status = "live"
		deployment.URL = fmt.Sprintf("http://localhost:%d", project.Port)
		deployment.Logs += fmt.Sprintf("\n\nContainer ID: %s", containerID)
		db.DB.Save(&deployment)
	}()
	c.JSON(http.StatusOK, gin.H{"status": "redeployment_started", "deployment_id": deployment.ID})
}
