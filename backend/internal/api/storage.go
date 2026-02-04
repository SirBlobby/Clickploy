package api

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"syscall"

	"clickploy/internal/db"
	"clickploy/internal/models"

	"github.com/gin-gonic/gin"
)

type CreateDatabaseRequest struct {
	Name string `json:"name" binding:"required"`
	Type string `json:"type" binding:"required,oneof=sqlite"`
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
	}
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
	userId := c.GetUint("userID")
	var dbs []models.Database
	if err := db.DB.Where("owner_id = ?", userId).Find(&dbs).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to list databases"})
		return
	}
	c.JSON(http.StatusOK, dbs)
}

func (h *Handler) handleCreateDatabase(c *gin.Context) {
	userId := c.GetUint("userID")
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

	dataDir := "./data/user_dbs"
	os.MkdirAll(dataDir, 0755)

	dbPath := filepath.Join(dataDir, fmt.Sprintf("%d_%s.db", userId, req.Name))
	file, err := os.Create(dbPath)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create database file"})
		return
	}
	file.Close()

	if err := db.DB.Create(&newDB).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to save database record"})
		return
	}

	c.JSON(http.StatusOK, newDB)
}
