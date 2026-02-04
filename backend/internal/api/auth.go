package api

import (
	"clickploy/internal/auth"
	"clickploy/internal/db"
	"clickploy/internal/models"
	"crypto/rand"
	"encoding/hex"
	"net/http"

	"github.com/gin-gonic/gin"
	gonanoid "github.com/matoous/go-nanoid/v2"
)

type AuthRequest struct {
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required,min=6"`
	Name     string `json:"name"`
}
type AuthResponse struct {
	Token string      `json:"token"`
	User  models.User `json:"user"`
}

func (h *Handler) RegisterAuthRoutes(r *gin.Engine) {
	authGroup := r.Group("/auth")
	{
		authGroup.POST("/register", h.register)
		authGroup.POST("/login", h.login)
	}
}
func (h *Handler) register(c *gin.Context) {
	var req AuthRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	hashed, err := auth.HashPassword(req.Password)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to hash password"})
		return
	}
	var count int64
	db.DB.Model(&models.User{}).Count(&count)
	apiKeyBytes := make([]byte, 32)
	rand.Read(apiKeyBytes)
	apiKey := "cp_" + hex.EncodeToString(apiKeyBytes)
	userID, _ := gonanoid.Generate("abcdefghijklmnopqrstuvwxyz0123456789", 10)
	user := models.User{
		ID:       userID,
		Email:    req.Email,
		Password: hashed,
		Name:     req.Name,
		Avatar:   "https://ui-avatars.com/api/?name=" + req.Name,
		IsAdmin:  count == 0,
		APIKey:   apiKey,
	}
	if result := db.DB.Create(&user); result.Error != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Email already exists"})
		return
	}
	token, _ := auth.GenerateToken(user.ID, user.Email)
	c.JSON(http.StatusCreated, AuthResponse{Token: token, User: user})
}
func (h *Handler) login(c *gin.Context) {
	var req AuthRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var user models.User
	if result := db.DB.Where("email = ?", req.Email).First(&user); result.Error != nil {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid credentials"})
		return
	}
	if !auth.CheckPassword(req.Password, user.Password) {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Invalid credentials"})
		return
	}
	token, _ := auth.GenerateToken(user.ID, user.Email)
	c.JSON(http.StatusOK, AuthResponse{Token: token, User: user})
}
func (h *Handler) RegisterUserRoutes(r *gin.Engine) {
	userGroup := r.Group("/api/user", AuthMiddleware())
	{
		userGroup.GET("", h.getMe)
		userGroup.PUT("/profile", h.updateProfile)
		userGroup.PUT("/password", h.updatePassword)
		userGroup.POST("/key", h.regenerateAPIKey)
	}
}
func (h *Handler) updateProfile(c *gin.Context) {
	userID, _ := c.Get("userID")
	var req struct {
		Name  string `json:"name"`
		Email string `json:"email"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var user models.User
	if result := db.DB.Where("id = ?", userID).First(&user); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "User not found"})
		return
	}
	if req.Name != "" {
		user.Name = req.Name
	}
	if req.Email != "" {
		user.Email = req.Email
	}
	db.DB.Save(&user)
	c.JSON(http.StatusOK, user)
}
func (h *Handler) updatePassword(c *gin.Context) {
	userID, _ := c.Get("userID")
	var req struct {
		OldPassword string `json:"old_password" binding:"required"`
		NewPassword string `json:"new_password" binding:"required,min=6"`
	}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	var user models.User
	if result := db.DB.Where("id = ?", userID).First(&user); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "User not found"})
		return
	}
	if !auth.CheckPassword(req.OldPassword, user.Password) {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Incorrect old password"})
		return
	}
	hashed, _ := auth.HashPassword(req.NewPassword)
	user.Password = hashed
	db.DB.Save(&user)
	c.JSON(http.StatusOK, gin.H{"message": "Password updated"})
}
func (h *Handler) getMe(c *gin.Context) {
	userID, _ := c.Get("userID")
	var user models.User
	if result := db.DB.Where("id = ?", userID).First(&user); result.Error != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "User not found"})
		return
	}
	c.JSON(http.StatusOK, user)
}
func (h *Handler) regenerateAPIKey(c *gin.Context) {
	userID, _ := c.Get("userID")
	apiKeyBytes := make([]byte, 32)
	rand.Read(apiKeyBytes)
	apiKey := "cp_" + hex.EncodeToString(apiKeyBytes)
	if err := db.DB.Model(&models.User{}).Where("id = ?", userID).Update("api_key", apiKey).Error; err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to update API key"})
		return
	}
	c.JSON(http.StatusOK, gin.H{"api_key": apiKey})
}
