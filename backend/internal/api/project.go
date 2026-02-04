package api

import (
	"bytes"
	"clickploy/internal/db"
	"clickploy/internal/models"
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	gonanoid "github.com/matoous/go-nanoid/v2"
	"gorm.io/gorm"
)

func (h *Handler) RegisterProjectRoutes(r *gin.Engine) {
	protected := r.Group("/api", AuthMiddleware())
	{
		protected.POST("/projects", h.createProject)
		protected.GET("/projects", h.listProjects)
		protected.GET("/projects/:id", h.getProject)
		protected.PUT("/projects/:id", h.updateProject)
		protected.PUT("/projects/:id/env", h.updateProjectEnv)
		protected.POST("/projects/:id/redeploy", h.redeployProject)
		protected.GET("/activity", h.getActivity)
	}
}
func (h *Handler) updateProject(c *gin.Context) {
	userID, _ := c.Get("userID")
	projectID := c.Param("id")
	var req struct {
		Name           string `json:"name"`
		BuildCommand   string `json:"build_command"`
		StartCommand   string `json:"start_command"`
		InstallCommand string `json:"install_command"`
		Runtime        string `json:"runtime"`
		GitToken       string `json:"git_token"`
		RepoURL        string `json:"repo_url"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var project models.Project
	if result := db.DB.Where("id = ? AND owner_id = ?", projectID, userID).First(&project); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}
	project.Name = req.Name
	project.BuildCommand = req.BuildCommand
	project.StartCommand = req.StartCommand
	project.InstallCommand = req.InstallCommand
	project.Runtime = req.Runtime
	project.RepoURL = req.RepoURL
	if req.GitToken != "" {
		project.GitToken = req.GitToken
	}
	if err := db.DB.Save(&project).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update project"})
		return
	}
	c.JSON(http.StatusOK, project)
}
func (h *Handler) updateProjectEnv(c *gin.Context) {
	userID, _ := c.Get("userID")
	projectID := c.Param("id")
	var req struct {
		EnvVars map[string]string `json:"env_vars"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var project models.Project
	if result := db.DB.Where("id = ? AND owner_id = ?", projectID, userID).First(&project); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}
	tx := db.DB.Begin()
	if err := tx.Where("project_id = ?", project.ID).Delete(&models.EnvVar{}).Error; err != nil {
		tx.Rollback()
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update env vars"})
		return
	}
	for k, v := range req.EnvVars {
		envVar := models.EnvVar{
			ProjectID: project.ID,
			Key:       k,
			Value:     v,
		}
		if err := tx.Create(&envVar).Error; err != nil {
			tx.Rollback()
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to save env var"})
			return
		}
	}
	tx.Commit()
	c.JSON(http.StatusOK, gin.H{"status": "updated"})
}
func (h *Handler) redeployProject(c *gin.Context) {
	userID, _ := c.Get("userID")
	projectID := c.Param("id")
	var req struct {
		Commit string `json:"commit"`
	}
	c.ShouldBindJSON(&req)
	var project models.Project
	if result := db.DB.Preload("EnvVars").Where("id = ? AND owner_id = ?", projectID, userID).First(&project); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}
	depID, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 10)
	initialCommit := "MANUAL"
	if req.Commit != "" {
		initialCommit = req.Commit
	}
	deployment := models.Deployment{
		ID:        depID,
		ProjectID: project.ID,
		Status:    "building",
		Commit:    initialCommit,
		Logs:      fmt.Sprintf("Starting redeploy for commit %s...", initialCommit),
	}
	db.DB.Create(&deployment)
	envMap := make(map[string]string)
	for _, env := range project.EnvVars {
		envMap[env.Key] = env.Value
	}
	resolvedEnv, err := h.resolveEnvVars(c.Request.Context(), userID.(string), envMap)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	go func() {
		var logBuffer bytes.Buffer
		streamer := &StreamWriter{DeploymentID: deployment.ID}
		multi := io.MultiWriter(&logBuffer, streamer)
		imageName, commitHash, err := h.builder.Build(project.RepoURL, req.Commit, project.Name, project.GitToken, project.BuildCommand, project.StartCommand, project.InstallCommand, project.Runtime, resolvedEnv, multi)
		deployment.Logs = logBuffer.String()
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nBuild Error: %v", err)
			db.DB.Save(&deployment)
			return
		}
		if commitHash != "" {
			deployment.Commit = commitHash
		}
		var envStrings []string
		for k, v := range resolvedEnv {
			envStrings = append(envStrings, fmt.Sprintf("%s=%s", k, v))
		}
		containerID, err := h.deployer.RunContainer(context.Background(), imageName, project.Name, project.Port, envStrings)
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nContainer Error: %v", err)
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
func (h *Handler) getActivity(c *gin.Context) {
	userID, _ := c.Get("userID")
	var deployments []models.Deployment
	err := db.DB.Joins("JOIN projects ON projects.id = deployments.project_id").
		Where("projects.owner_id = ?", userID).
		Order("deployments.created_at desc").
		Limit(20).
		Preload("Project").
		Find(&deployments).Error
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch activity"})
		return
	}
	c.JSON(http.StatusOK, deployments)
}
func (h *Handler) createProject(c *gin.Context) {
	userID, _ := c.Get("userID")
	var req struct {
		DeployRequest
		EnvVars        map[string]string `json:"env_vars"`
		BuildCommand   string            `json:"build_command"`
		StartCommand   string            `json:"start_command"`
		InstallCommand string            `json:"install_command"`
		Runtime        string            `json:"runtime"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	port, err := h.ports.GetPort(req.Name, req.Port)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Port allocation failed: %v", err)})
		return
	}
	resolvedEnv, err := h.resolveEnvVars(c.Request.Context(), userID.(string), req.EnvVars)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var envVarsModel []models.EnvVar
	for k, v := range req.EnvVars {
		envVarsModel = append(envVarsModel, models.EnvVar{Key: k, Value: v})
	}
	secretBytes := make([]byte, 16)
	rand.Read(secretBytes)
	webhookSecret := hex.EncodeToString(secretBytes)
	id, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 10)
	project := models.Project{
		ID:             id,
		Name:           req.Name,
		RepoURL:        req.Repo,
		OwnerID:        userID.(string),
		Port:           port,
		WebhookSecret:  webhookSecret,
		GitToken:       req.GitToken,
		EnvVars:        envVarsModel,
		BuildCommand:   req.BuildCommand,
		StartCommand:   req.StartCommand,
		InstallCommand: req.InstallCommand,
		Runtime:        req.Runtime,
	}
	if result := db.DB.Create(&project); result.Error != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to save project to DB"})
		return
	}
	depID, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 10)
	deployment := models.Deployment{
		ID:        depID,
		ProjectID: project.ID,
		Status:    "building",
		Commit:    "HEAD",
		Logs:      "Starting build...",
	}
	db.DB.Create(&deployment)
	go func() {
		var logBuffer bytes.Buffer
		streamer := &StreamWriter{DeploymentID: deployment.ID}
		multi := io.MultiWriter(&logBuffer, streamer)
		imageName, commitHash, err := h.builder.Build(req.Repo, "", req.Name, req.GitToken, req.BuildCommand, req.StartCommand, req.InstallCommand, req.Runtime, resolvedEnv, multi)
		deployment.Logs = logBuffer.String()
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nBuild Error: %v", err)
			db.DB.Save(&deployment)
			return
		}
		if commitHash != "" {
			deployment.Commit = commitHash
		}
		var envStrings []string
		for k, v := range resolvedEnv {
			envStrings = append(envStrings, fmt.Sprintf("%s=%s", k, v))
		}
		containerID, err := h.deployer.RunContainer(context.Background(), imageName, req.Name, port, envStrings)
		if err != nil {
			deployment.Status = "failed"
			deployment.Logs += fmt.Sprintf("\n\nContainer Error: %v", err)
			db.DB.Save(&deployment)
			return
		}
		deployment.Status = "live"
		deployment.URL = fmt.Sprintf("http://localhost:%d", port)
		deployment.Logs += fmt.Sprintf("\n\nContainer ID: %s", containerID)
		db.DB.Save(&deployment)
	}()
	project.Deployments = []models.Deployment{deployment}
	c.JSON(http.StatusOK, project)
}
func (h *Handler) resolveEnvVars(ctx context.Context, userID string, envVars map[string]string) (map[string]string, error) {
	resolved := make(map[string]string)
	for k, v := range envVars {
		if strings.HasPrefix(v, "$DB:") {
			dbName := strings.TrimPrefix(v, "$DB:")
			var database models.Database
			if err := db.DB.Where("name = ? AND owner_id = ?", dbName, userID).First(&database).Error; err != nil {
				return nil, fmt.Errorf("database '%s' not found", dbName)
			}
			if database.Type != "mongodb" {
				return nil, fmt.Errorf("database '%s' is not a mongodb instance (auto-resolution only supported for mongo)", dbName)
			}
			env, err := h.deployer.GetContainerEnv(ctx, database.ContainerID)
			if err != nil {
				return nil, fmt.Errorf("failed to get env for database '%s': %v", dbName, err)
			}
			var user, pass string
			for _, e := range env {
				if len(e) > 25 && e[:25] == "MONGO_INITDB_ROOT_USERNAME=" {
					user = e[25:]
				} else if len(e) > 25 && e[:25] == "MONGO_INITDB_ROOT_PASSWORD=" {
					pass = e[25:]
				}
			}
			resolved[k] = fmt.Sprintf("mongodb://%s:%s@172.17.0.1:%d/?authSource=admin", user, pass, database.Port)
		} else {
			resolved[k] = v
		}
	}
	return resolved, nil
}
func (h *Handler) listProjects(c *gin.Context) {
	userID, _ := c.Get("userID")
	var projects []models.Project
	if result := db.DB.Preload("Deployments").Where("owner_id = ?", userID).Find(&projects); result.Error != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch projects"})
		return
	}
	c.JSON(http.StatusOK, projects)
}
func (h *Handler) getProject(c *gin.Context) {
	userID, _ := c.Get("userID")
	projectID := c.Param("id")
	var project models.Project
	if result := db.DB.Order("created_at desc").Preload("Deployments", func(db *gorm.DB) *gorm.DB {
		return db.Order("deployments.created_at desc")
	}).Preload("EnvVars").Where("id = ? AND owner_id = ?", projectID, userID).First(&project); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Project not found"})
		return
	}
	c.JSON(http.StatusOK, project)
}
