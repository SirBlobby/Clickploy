use anyhow::{Context, Result};
use reqwest::Client;
use crate::models::*;

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
        }
    }

    pub async fn validate_connection(&self) -> Result<User> {
        let url = format!("{}/api/user", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to connect to server")?;

        if !response.status().is_success() {
            anyhow::bail!("Authentication failed: {}", response.status());
        }

        let user = response.json::<User>().await
            .context("Failed to parse user response")?;

        Ok(user)
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let url = format!("{}/api/projects", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch projects")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch projects: {}", response.status());
        }

        let projects = response.json::<Vec<Project>>().await
            .context("Failed to parse projects response")?;

        Ok(projects)
    }

    pub async fn get_project(&self, id: &str) -> Result<Project> {
        let url = format!("{}/api/projects/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch project")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch project: {}", response.status());
        }

        let project = response.json::<Project>().await
            .context("Failed to parse project response")?;

        Ok(project)
    }

    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<Project> {
        let url = format!("{}/api/projects", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to create project")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create project: {}", error_text);
        }

        let project = response.json::<Project>().await
            .context("Failed to parse project response")?;

        Ok(project)
    }

    pub async fn redeploy_project(&self, id: &str, commit: Option<String>) -> Result<serde_json::Value> {
        let url = format!("{}/api/projects/{}/redeploy", self.base_url, id);
        
        let request = RedeployRequest { commit };
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to redeploy project")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to redeploy project: {}", response.status());
        }

        let result = response.json::<serde_json::Value>().await
            .context("Failed to parse redeploy response")?;

        Ok(result)
    }

    pub async fn stop_project(&self, id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/api/projects/{}/stop", self.base_url, id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to stop project")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to stop project: {}", response.status());
        }

        let result = response.json::<serde_json::Value>().await
            .context("Failed to parse stop response")?;

        Ok(result)
    }

    pub async fn get_activity(&self) -> Result<Vec<Deployment>> {
        let url = format!("{}/api/activity", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch activity")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch activity: {}", response.status());
        }

        let deployments = response.json::<Vec<Deployment>>().await
            .context("Failed to parse activity response")?;

        Ok(deployments)
    }

    // Storage API methods
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        let url = format!("{}/api/storage/stats", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch storage stats")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch storage stats: {}", response.status());
        }

        let stats = response.json::<StorageStats>().await
            .context("Failed to parse storage stats response")?;

        Ok(stats)
    }

    pub async fn list_databases(&self) -> Result<Vec<Database>> {
        let url = format!("{}/api/storage/databases", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch databases")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch databases: {}", response.status());
        }

        let databases = response.json::<Vec<Database>>().await
            .context("Failed to parse databases response")?;

        Ok(databases)
    }

    pub async fn create_database(&self, name: String, db_type: String) -> Result<serde_json::Value> {
        let url = format!("{}/api/storage/databases", self.base_url);
        
        let request = CreateDatabaseRequest { name, db_type };
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to create database")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to create database: {}", error_text);
        }

        let result = response.json::<serde_json::Value>().await
            .context("Failed to parse create database response")?;

        Ok(result)
    }

    pub async fn delete_database(&self, id: u32) -> Result<()> {
        let url = format!("{}/api/storage/databases/{}", self.base_url, id);
        
        let response = self.client
            .delete(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to delete database")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete database: {}", response.status());
        }

        Ok(())
    }

    pub async fn get_database_credentials(&self, id: u32) -> Result<DatabaseCredentials> {
        let url = format!("{}/api/storage/databases/{}/credentials", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to fetch database credentials")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch database credentials: {}", response.status());
        }

        let credentials = response.json::<DatabaseCredentials>().await
            .context("Failed to parse database credentials response")?;

        Ok(credentials)
    }

    pub async fn update_database_credentials(&self, id: u32, username: String, password: String) -> Result<()> {
        let url = format!("{}/api/storage/databases/{}/credentials", self.base_url, id);
        
        let request = UpdateDatabaseCredentialsRequest { username, password };
        
        let response = self.client
            .put(&url)
            .header("Authorization", &self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to update database credentials")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update database credentials: {}", response.status());
        }

        Ok(())
    }

    pub async fn stop_database(&self, id: u32) -> Result<()> {
        let url = format!("{}/api/storage/databases/{}/stop", self.base_url, id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to stop database")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to stop database: {}", response.status());
        }

        Ok(())
    }

    pub async fn restart_database(&self, id: u32) -> Result<()> {
        let url = format!("{}/api/storage/databases/{}/restart", self.base_url, id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", &self.api_key)
            .send()
            .await
            .context("Failed to restart database")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to restart database: {}", response.status());
        }

        Ok(())
    }
}
