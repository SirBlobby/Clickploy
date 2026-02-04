use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub avatar: String,
    pub is_admin: bool,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub repo_url: String,
    pub port: i32,
    pub owner_id: String,
    pub webhook_secret: String,
    pub build_command: String,
    pub start_command: String,
    pub install_command: String,
    pub runtime: String,
    pub created_at: String,
    pub updated_at: String,
    pub deployments: Option<Vec<Deployment>>,
    pub env_vars: Option<Vec<EnvVar>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Deployment {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub commit: String,
    pub logs: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EnvVar {
    #[serde(rename = "ID")]
    pub id: u32,
    pub project_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub repo: String,
    pub port: Option<i32>,
    pub git_token: Option<String>,
    pub env_vars: Option<std::collections::HashMap<String, String>>,
    pub build_command: Option<String>,
    pub start_command: Option<String>,
    pub install_command: Option<String>,
    pub runtime: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RedeployRequest {
    pub commit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "DeletedAt")]
    pub deleted_at: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub db_type: String,
    pub status: String,
    pub owner_id: String,
    pub size_mb: f64,
    pub container_id: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct StorageStats {
    pub used: u64,
    pub total: u64,
    pub percent: f64,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub uri: String,
    pub public_uri: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub db_type: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateDatabaseCredentialsRequest {
    pub username: String,
    pub password: String,
}
