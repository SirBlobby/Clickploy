package api

import (
	"clickploy/internal/db"
	"clickploy/internal/models"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"syscall"

	"github.com/gin-gonic/gin"
	gonanoid "github.com/matoous/go-nanoid/v2"
)

type CreateDatabaseRequest struct {
	Name string `json:"name" binding:"required"`
	Type string `json:"type" binding:"required,oneof=sqlite mongodb"`
	Port int    `json:"port"`
}
type StorageStatsResponse struct {
	TotalMB float64 `json:"total_mb"`
	UsedMB  float64 `json:"used_mb"`
}

func (h *Handler) RegisterStorageRoutes(r *gin.Engine) {
	api := r.Group("/api/storage")
	api.Use(AuthMiddleware())
	{
		api.GET("/stats", h.handleGetStorageStats)
		api.GET("/databases", h.handleListDatabases)
		api.POST("/databases", h.handleCreateDatabase)
		api.PUT("/databases/:id", h.handleUpdateDatabase)
		api.DELETE("/databases/:id", h.handleDeleteDatabase)
		api.GET("/databases/:id/credentials", h.handleGetDatabaseCredentials)
		api.PUT("/databases/:id/credentials", h.handleUpdateDatabaseCredentials)
		api.POST("/databases/:id/stop", h.handleStopDatabase)
		api.POST("/databases/:id/restart", h.handleRestartDatabase)
	}
}
func (h *Handler) handleUpdateDatabase(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var req struct {
		Port int `json:"port"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type == "mongodb" && req.Port != 0 && req.Port != database.Port {
		envVars, err := h.deployer.GetContainerEnv(c.Request.Context(), database.ContainerID)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to retrieve current container configuration"})
			return
		}
		wd, _ := os.Getwd()
		volumePath := filepath.Join(wd, "data", "volumes", fmt.Sprintf("%s_%s", userId, database.Name))
		containerName := fmt.Sprintf("mongo_%s_%s", userId, database.Name)
		newId, err := h.deployer.StartDatabaseContainer(c.Request.Context(), "mongo:latest", containerName, req.Port, volumePath, envVars)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to restart container: %v", err)})
			return
		}
		database.Port = req.Port
		database.ContainerID = newId
		if err := db.DB.Save(&database).Error; err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update database record"})
			return
		}
	}
	c.JSON(http.StatusOK, database)
}
func (h *Handler) handleGetStorageStats(c *gin.Context) {
	var stat syscall.Statfs_t
	wd, _ := os.Getwd()
	if err := syscall.Statfs(wd, &stat); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to get disk stats"})
		return
	}
	totalBytes := stat.Blocks * uint64(stat.Bsize)
	availBytes := stat.Bavail * uint64(stat.Bsize)
	usedBytes := totalBytes - availBytes
	c.JSON(http.StatusOK, StorageStatsResponse{
		TotalMB: float64(totalBytes) / 1024 / 1024,
		UsedMB:  float64(usedBytes) / 1024 / 1024,
	})
}
func (h *Handler) handleListDatabases(c *gin.Context) {
	userId := c.GetString("userID")
	var dbs []models.Database
	if err := db.DB.Where("owner_id = ?", userId).Find(&dbs).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to list databases"})
		return
	}
	c.JSON(http.StatusOK, dbs)
}
func (h *Handler) handleCreateDatabase(c *gin.Context) {
	userId := c.GetString("userID")
	var req CreateDatabaseRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	newDB := models.Database{
		Name:    req.Name,
		Type:    req.Type,
		Status:  "available",
		OwnerID: userId,
		SizeMB:  0,
	}
	if req.Type == "sqlite" {
		dataDir := "./data/user_dbs"
		os.MkdirAll(dataDir, 0755)
		dbPath := filepath.Join(dataDir, fmt.Sprintf("%s_%s.db", userId, req.Name))
		file, err := os.Create(dbPath)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create database file"})
			return
		}
		file.Close()
	} else if req.Type == "mongodb" {
		port := 27017
		if req.Port != 0 {
			p, err := h.ports.GetPort(req.Name, req.Port)
			if err != nil {
				c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Port %d is not available", req.Port)})
				return
			}
			port = p
		} else {
			p, err := h.ports.GetPort(req.Name, 27017)
			if err == nil {
				port = p
			} else {
				p, err := h.ports.GetPort(req.Name, 0)
				if err != nil {
					c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to allocate port"})
					return
				}
				port = p
			}
		}
		newDB.Port = port
		username := "root"
		password, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 16)
		wd, _ := os.Getwd()
		volumePath := filepath.Join(wd, "data", "volumes", fmt.Sprintf("%s_%s", userId, req.Name))
		os.MkdirAll(volumePath, 0755)
		containerName := fmt.Sprintf("mongo_%s_%s", userId, req.Name)
		envVars := []string{
			fmt.Sprintf("MONGO_INITDB_ROOT_USERNAME=%s", username),
			fmt.Sprintf("MONGO_INITDB_ROOT_PASSWORD=%s", password),
		}
		id, err := h.deployer.StartDatabaseContainer(c.Request.Context(), "mongo:latest", containerName, port, volumePath, envVars)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to start database container: %v", err)})
			return
		}
		newDB.ContainerID = id
		newDB.Status = "running"
		c.JSON(http.StatusOK, gin.H{
			"database": newDB,
			"username": username,
			"password": password,
			"uri":      fmt.Sprintf("mongodb://%s:%s@localhost:%d/?authSource=admin", username, password, port),
		})
		if err := db.DB.Create(&newDB).Error; err != nil {
			fmt.Printf("Failed to save DB record: %v\n", err)
		}
		return
	}
	if err := db.DB.Create(&newDB).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to save database record"})
		return
	}
	c.JSON(http.StatusOK, newDB)
}
func (h *Handler) handleDeleteDatabase(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type == "sqlite" {
		dataDir := "./data/user_dbs"
		dbPath := filepath.Join(dataDir, fmt.Sprintf("%s_%s.db", userId, database.Name))
		if err := os.Remove(dbPath); err != nil && !os.IsNotExist(err) {
			fmt.Printf("Failed to delete database file: %v\n", err)
		}
	} else if database.Type == "mongodb" {
		if database.ContainerID != "" {
			if err := h.deployer.RemoveContainer(c.Request.Context(), database.ContainerID); err != nil {
				fmt.Printf("Failed to remove container: %v\n", err)
			}
		}
		wd, _ := os.Getwd()
		volumePath := filepath.Join(wd, "data", "volumes", fmt.Sprintf("%s_%s", userId, database.Name))
		os.RemoveAll(volumePath)
	}
	if err := db.DB.Delete(&database).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to delete database record"})
		return
	}
	c.JSON(http.StatusOK, gin.H{"status": "deleted"})
}
func (h *Handler) handleGetDatabaseCredentials(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type != "mongodb" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Credential management only supported for MongoDB"})
		return
	}
	envVars, err := h.deployer.GetContainerEnv(c.Request.Context(), database.ContainerID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to retrieve container configuration"})
		return
	}
	var username, password string
	fmt.Printf("Debug: Container env vars count: %d\n", len(envVars))
	for _, env := range envVars {
		fmt.Printf("Debug: env var: %s\n", env)
		if len(env) > 26 && env[:26] == "MONGO_INITDB_ROOT_USERNAME=" {
			username = env[26:]
			fmt.Printf("Debug: Found username: %s\n", username)
		} else if len(env) > 26 && env[:26] == "MONGO_INITDB_ROOT_PASSWORD=" {
			password = env[26:]
			fmt.Printf("Debug: Found password: %s\n", password)
		}
	}
	fmt.Printf("Debug: Final username=%s, password=%s\n", username, password)
	uri := fmt.Sprintf("mongodb://%s:%s@localhost:%d/?authSource=admin", username, password, database.Port)
	publicUri := fmt.Sprintf("mongodb://%s:%s@<HOST>:%d/?authSource=admin", username, password, database.Port)
	c.JSON(http.StatusOK, gin.H{
		"username":   username,
		"password":   password,
		"uri":        uri,
		"public_uri": publicUri,
	})
}
func (h *Handler) handleUpdateDatabaseCredentials(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var req struct {
		Username string `json:"username" binding:"required"`
		Password string `json:"password" binding:"required"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type != "mongodb" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Credential management only supported for MongoDB"})
		return
	}
	envVars, err := h.deployer.GetContainerEnv(c.Request.Context(), database.ContainerID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to retrieve container configuration"})
		return
	}
	var currentUser, currentPass string
	var otherEnv []string
	for _, env := range envVars {
		if len(env) > 26 && env[:26] == "MONGO_INITDB_ROOT_USERNAME=" {
			currentUser = env[26:]
		} else if len(env) > 26 && env[:26] == "MONGO_INITDB_ROOT_PASSWORD=" {
			currentPass = env[26:]
		} else {
			otherEnv = append(otherEnv, env)
		}
	}
	if currentUser == "" || currentPass == "" {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Could not determine current credentials from container"})
		return
	}
	ctx := c.Request.Context()
	needsRestart := false
	if req.Username != currentUser {
		createCmd := fmt.Sprintf("db.getSiblingDB('admin').createUser({user: '%s', pwd: '%s', roles: ['root']})", req.Username, req.Password)
		if err := h.deployer.ExecContainer(ctx, database.ContainerID, []string{"mongosh", "admin", "-u", currentUser, "-p", currentPass, "--eval", createCmd}); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to create new user: %v", err)})
			return
		}
		dropCmd := fmt.Sprintf("db.getSiblingDB('admin').dropUser('%s')", currentUser)
		if err := h.deployer.ExecContainer(ctx, database.ContainerID, []string{"mongosh", "admin", "-u", req.Username, "-p", req.Password, "--eval", dropCmd}); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to drop old user: %v", err)})
			return
		}
		needsRestart = true
	} else if req.Password != currentPass {
		changeCmd := fmt.Sprintf("db.getSiblingDB('admin').changeUserPassword('%s', '%s')", currentUser, req.Password)
		if err := h.deployer.ExecContainer(ctx, database.ContainerID, []string{"mongosh", "admin", "-u", currentUser, "-p", currentPass, "--eval", changeCmd}); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to change password: %v", err)})
			return
		}
		needsRestart = true
	}
	if needsRestart {
		newEnv := append(otherEnv,
			fmt.Sprintf("MONGO_INITDB_ROOT_USERNAME=%s", req.Username),
			fmt.Sprintf("MONGO_INITDB_ROOT_PASSWORD=%s", req.Password),
		)
		wd, _ := os.Getwd()
		volumePath := filepath.Join(wd, "data", "volumes", fmt.Sprintf("%s_%s", userId, database.Name))
		containerName := fmt.Sprintf("mongo_%s_%s", userId, database.Name)
		newId, err := h.deployer.StartDatabaseContainer(ctx, "mongo:latest", containerName, database.Port, volumePath, newEnv)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to restart container with new creds: %v", err)})
			return
		}
		database.ContainerID = newId
		if err := db.DB.Save(&database).Error; err != nil {
		}
	}
	c.JSON(http.StatusOK, gin.H{"status": "updated"})
}
func (h *Handler) handleStopDatabase(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type == "mongodb" && database.ContainerID != "" {
		if err := h.deployer.StopContainer(c.Request.Context(), database.ContainerID); err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to stop container: %v", err)})
			return
		}
		database.Status = "stopped"
		if err := db.DB.Save(&database).Error; err != nil {
			fmt.Printf("Failed to update db status: %v\n", err)
		}
	}
	c.JSON(http.StatusOK, gin.H{"status": "stopped"})
}
func (h *Handler) handleRestartDatabase(c *gin.Context) {
	userId := c.GetString("userID")
	dbId := c.Param("id")
	var database models.Database
	if err := db.DB.Where("id = ? AND owner_id = ?", dbId, userId).First(&database).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Database not found"})
		return
	}
	if database.Type == "mongodb" && database.ContainerID != "" {
		if err := h.deployer.RestartContainer(c.Request.Context(), database.ContainerID); err != nil {
			if err := h.deployer.StartContainer(c.Request.Context(), database.ContainerID); err != nil {
				c.JSON(http.StatusInternalServerError, gin.H{"error": fmt.Sprintf("Failed to restart container: %v", err)})
				return
			}
		}
		database.Status = "running"
		if err := db.DB.Save(&database).Error; err != nil {
			fmt.Printf("Failed to update db status: %v\n", err)
		}
	}
	c.JSON(http.StatusOK, gin.H{"status": "restarted"})
}
