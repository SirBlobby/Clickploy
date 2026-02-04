package models

import (
	"time"

	"gorm.io/gorm"
)

type User struct {
	ID        string         `gorm:"primaryKey" json:"id"`
	CreatedAt time.Time      `json:"created_at"`
	UpdatedAt time.Time      `json:"updated_at"`
	DeletedAt gorm.DeletedAt `gorm:"index" json:"deleted_at"`
	Email     string         `gorm:"uniqueIndex;not null" json:"email"`
	Password  string         `json:"-"`
	Name      string         `json:"name"`
	Avatar    string         `json:"avatar"`
	IsAdmin   bool           `json:"is_admin"`
	Projects  []Project      `gorm:"foreignKey:OwnerID" json:"projects"`
}

type Project struct {
	ID             string         `gorm:"primaryKey" json:"id"`
	CreatedAt      time.Time      `json:"created_at"`
	UpdatedAt      time.Time      `json:"updated_at"`
	DeletedAt      gorm.DeletedAt `gorm:"index" json:"deleted_at"`
	Name           string         `gorm:"uniqueIndex" json:"name"`
	RepoURL        string         `json:"repo_url"`
	OwnerID        string         `json:"owner_id"`
	Port           int            `json:"port"`
	WebhookSecret  string         `json:"webhook_secret"`
	GitToken       string         `json:"-"`
	BuildCommand   string         `json:"build_command"`
	StartCommand   string         `json:"start_command"`
	InstallCommand string         `json:"install_command"`
	Runtime        string         `json:"runtime"`
	Deployments    []Deployment   `gorm:"foreignKey:ProjectID" json:"deployments"`
	EnvVars        []EnvVar       `gorm:"foreignKey:ProjectID" json:"env_vars"`
}

type EnvVar struct {
	gorm.Model
	ProjectID string `json:"project_id"`
	Key       string `json:"key"`
	Value     string `json:"value"`
}

type Deployment struct {
	ID        string         `gorm:"primaryKey" json:"id"`
	CreatedAt time.Time      `json:"created_at"`
	UpdatedAt time.Time      `json:"updated_at"`
	DeletedAt gorm.DeletedAt `gorm:"index" json:"deleted_at"`
	ProjectID string         `json:"project_id"`
	Project   Project        `json:"project" gorm:"constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
	Status    string         `json:"status"`
	Commit    string         `json:"commit"`
	Logs      string         `json:"logs"`
	URL       string         `json:"url"`
}

type Database struct {
	gorm.Model
	Name    string  `json:"name"`
	Type    string  `json:"type"`
	Status  string  `json:"status"`
	OwnerID string  `json:"owner_id"`
	SizeMB  float64 `json:"size_mb"`
}
