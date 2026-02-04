package api

import (
	"net/http"

	"clickploy/internal/db"
	"clickploy/internal/models"

	"github.com/gin-gonic/gin"
)

func (h *Handler) RegisterAdminRoutes(r *gin.Engine) {
	admin := r.Group("/api/admin", AuthMiddleware(), AdminMiddleware())
	{
		admin.GET("/users", h.adminListUsers)
		admin.DELETE("/users/:id", h.adminDeleteUser)
		admin.GET("/stats", h.adminGetStats)
	}
}

func (h *Handler) adminListUsers(c *gin.Context) {
	var users []models.User
	if err := db.DB.Preload("Projects").Find(&users).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch users"})
		return
	}
	c.JSON(http.StatusOK, users)
}

func (h *Handler) adminDeleteUser(c *gin.Context) {
	id := c.Param("id")
	if err := db.DB.Where("id = ?", id).Delete(&models.User{}).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to delete user"})
		return
	}
	c.JSON(http.StatusOK, gin.H{"status": "deleted"})
}

func (h *Handler) adminGetStats(c *gin.Context) {
	var userCount int64
	var projectCount int64
	var deploymentCount int64

	db.DB.Model(&models.User{}).Count(&userCount)
	db.DB.Model(&models.Project{}).Count(&projectCount)
	db.DB.Model(&models.Deployment{}).Count(&deploymentCount)

	c.JSON(http.StatusOK, gin.H{
		"users":       userCount,
		"projects":    projectCount,
		"deployments": deploymentCount,
	})
}
