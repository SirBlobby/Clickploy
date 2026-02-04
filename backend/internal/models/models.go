package models

import (
	"gorm.io/gorm"
)

type User struct {
	gorm.Model
	Email    string    `gorm:"uniqueIndex;not null" json:"email"`
	Password string    `json:"-"`
	Name     string    `json:"name"`
	Avatar   string    `json:"avatar"`
	Projects []Project `gorm:"foreignKey:OwnerID" json:"projects"`
}

type Project struct {
	gorm.Model
	Name           string       `gorm:"uniqueIndex" json:"name"`
	RepoURL        string       `json:"repo_url"`
	OwnerID        uint         `json:"owner_id"`
	Port           int          `json:"port"`
	WebhookSecret  string       `json:"webhook_secret"`
	GitToken       string       `json:"-"`
	BuildCommand   string       `json:"build_command"`
	StartCommand   string       `json:"start_command"`
	InstallCommand string       `json:"install_command"`
	Runtime        string       `json:"runtime"`
	Deployments    []Deployment `gorm:"foreignKey:ProjectID" json:"deployments"`
	EnvVars        []EnvVar     `gorm:"foreignKey:ProjectID" json:"env_vars"`
}

type EnvVar struct {
	gorm.Model
	ProjectID uint   `json:"project_id"`
	Key       string `json:"key"`
	Value     string `json:"value"`
}

type Deployment struct {
	gorm.Model
	ProjectID uint    `json:"project_id"`
	Project   Project `json:"project" gorm:"constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
	Status    string  `json:"status"`
	Commit    string  `json:"commit"`
	Logs      string  `json:"logs"`
	URL       string  `json:"url"`
}

type Database struct {
	gorm.Model
	Name    string  `json:"name"`
	Type    string  `json:"type"`
	Status  string  `json:"status"`
	OwnerID uint    `json:"owner_id"`
	SizeMB  float64 `json:"size_mb"`
}
