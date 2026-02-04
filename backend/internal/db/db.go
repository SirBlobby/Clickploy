package db
import (
	"log"
	"path/filepath"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"clickploy/internal/models"
)
var DB *gorm.DB
func Init(storagePath string) {
	var err error
	dbPath := filepath.Join(storagePath, "clickploy.db")
	DB, err = gorm.Open(sqlite.Open(dbPath), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})
	if err != nil {
		log.Fatal("Failed to connect to database:", err)
	}
	log.Println("Migrating database...")
	err = DB.AutoMigrate(&models.User{}, &models.Project{}, &models.Deployment{}, &models.EnvVar{}, &models.Database{})
	if err != nil {
		log.Fatal("Failed to migrate database:", err)
	}
	log.Println("Database initialized successfully")
}
