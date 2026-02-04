package main

import (
	"log"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"

	"clickploy/internal/api"
	"clickploy/internal/builder"
	"clickploy/internal/db"
	"clickploy/internal/deployer"
	"clickploy/internal/ports"
)

func main() {
	db.Init(".")
	pm := ports.NewManager(4000, 5000)
	buildr := builder.NewBuilder()
	dply, err := deployer.NewDeployer()
	if err != nil {
		log.Fatalf("Failed to create deployer: %v", err)
	}

	handler := api.NewHandler(buildr, dply, pm)

	r := gin.Default()

	r.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	handler.RegisterRoutes(r)
	handler.RegisterAuthRoutes(r)
	handler.RegisterUserRoutes(r)
	handler.RegisterProjectRoutes(r)
	handler.RegisterWebhookRoutes(r)
	handler.RegisterSystemRoutes(r)
	handler.RegisterStorageRoutes(r)
	handler.RegisterAdminRoutes(r)

	log.Println("Starting Clickploy Backend on :8080")
	if err := r.Run(":8080"); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
