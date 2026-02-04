package api

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"strconv"

	"clickploy/internal/db"
	"clickploy/internal/models"

	"github.com/gin-gonic/gin"
)

func (h *Handler) RegisterWebhookRoutes(r *gin.Engine) {
	r.POST("/webhooks/trigger", h.handleWebhook)
}

func (h *Handler) handleWebhook(c *gin.Context) {
	projectIDHex := c.Query("project_id")
	if projectIDHex == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "project_id required"})
		return
	}

	pid, err := strconv.ParseUint(projectIDHex, 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid Project ID"})
		return
	}

	var project models.Project
	if result := db.DB.Preload("EnvVars").First(&project, pid); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}

	deployment := models.Deployment{
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

		imageName, err := h.builder.Build(project.RepoURL, project.Name, project.GitToken, project.BuildCommand, project.StartCommand, project.InstallCommand, project.Runtime, envMap, multi)
		deployment.Logs = logBuffer.String()
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nBuild Failed: %v", err)
			db.DB.Save(&deployment)
			return
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
