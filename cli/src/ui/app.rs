use crate::api::ApiClient;
use crate::models::{Deployment, Project, User, Database, StorageStats, DatabaseCredentials};
use crate::ui::{CreateProjectState, CreateDatabaseState};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Setup,
    Projects,
    CreateProject,
    CreateDatabase,
    ProjectDetail(String), // project ID
    ProjectSettings(String), // project ID
    DeploymentLogs(String), // deployment ID
    Deployments,
    Activity,
    Network,
    Storage,
    Docs,
    Settings,
}

pub struct App {
    pub screen: Screen,
    pub projects: Vec<Project>,
    pub selected_project: Option<Project>,
    pub activity: Vec<Deployment>,
    pub user: Option<User>,
    pub message: String,
    pub error: Option<String>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub create_project_state: CreateProjectState,
    pub live_logs: String,
    pub log_scroll: u16,
    pub databases: Vec<Database>,
    pub selected_database: Option<Database>,
    pub storage_stats: Option<StorageStats>,
    pub create_database_state: CreateDatabaseState,
    pub show_db_credentials: bool,
    pub db_credentials: Option<DatabaseCredentials>,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Setup,
            projects: Vec::new(),
            selected_project: None,
            activity: Vec::new(),
            user: None,
            message: String::new(),
            error: None,
            selected_index: 0,
            should_quit: false,
            create_project_state: CreateProjectState::new(),
            live_logs: String::new(),
            log_scroll: 0,
            databases: Vec::new(),
            selected_database: None,
            storage_stats: None,
            create_database_state: CreateDatabaseState::new(),
            show_db_credentials: false,
            db_credentials: None,
        }
    }

    pub async fn fetch_projects(&mut self, client: &ApiClient) -> Result<()> {
        self.message = "Fetching projects...".to_string();
        self.error = None;
        
        match client.list_projects().await {
            Ok(projects) => {
                self.projects = projects;
                self.message = format!("Loaded {} projects", self.projects.len());
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch projects: {}", e));
            }
        }
        
        Ok(())
    }

    pub async fn fetch_project_detail(&mut self, client: &ApiClient, id: &str) -> Result<()> {
        self.error = None;
        
        match client.get_project(id).await {
            Ok(project) => {
                self.selected_project = Some(project);
                self.message.clear();
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch project: {}", e));
            }
        }
        
        Ok(())
    }

    pub async fn fetch_activity(&mut self, client: &ApiClient) -> Result<()> {
        self.message = "Fetching activity...".to_string();
        self.error = None;
        
        match client.get_activity().await {
            Ok(activity) => {
                self.activity = activity;
                self.message = format!("Loaded {} deployments", self.activity.len());
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch activity: {}", e));
            }
        }
        
        Ok(())
    }

    pub async fn fetch_storage_data(&mut self, client: &ApiClient) -> Result<()> {
        self.message = "Fetching storage data...".to_string();
        self.error = None;
        
        match client.list_databases().await {
            Ok(databases) => {
                self.databases = databases;
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch databases: {}", e));
            }
        }
        
        match client.get_storage_stats().await {
            Ok(stats) => {
                self.storage_stats = Some(stats);
                self.message = format!("Loaded {} databases", self.databases.len());
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch storage stats: {}", e));
            }
        }
        
        Ok(())
    }

    pub fn next(&mut self) {
        let len = match self.screen {
            Screen::Projects => self.projects.len(),
            Screen::Activity => self.activity.len(),
            Screen::Storage => self.databases.len(),
            _ => 0,
        };
        
        if len > 0 {
            self.selected_index = (self.selected_index + 1) % len;
        }
    }

    pub fn previous(&mut self) {
        let len = match self.screen {
            Screen::Projects => self.projects.len(),
            Screen::Activity => self.activity.len(),
            Screen::Storage => self.databases.len(),
            _ => 0,
        };
        
        if len > 0 {
            if self.selected_index == 0 {
                self.selected_index = len - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn select_project(&mut self) {
        if self.selected_index < self.projects.len() {
            let project_id = self.projects[self.selected_index].id.clone();
            self.screen = Screen::ProjectDetail(project_id);
            self.selected_index = 0;
            self.message.clear();
        }
    }

    pub fn select_database(&mut self) {
        if self.selected_index < self.databases.len() {
            self.selected_database = Some(self.databases[self.selected_index].clone());
            self.show_db_credentials = true;
        }
    }

    pub fn go_back(&mut self) {
        match &self.screen {
            Screen::ProjectDetail(_) | Screen::ProjectSettings(_) | Screen::Activity | Screen::Settings | Screen::CreateProject | Screen::CreateDatabase | Screen::Deployments | Screen::Network | Screen::Storage | Screen::Docs => {
                self.screen = Screen::Projects;
                self.selected_index = 0;
                self.message.clear();
                self.show_db_credentials = false;
                self.db_credentials = None;
            }
            Screen::DeploymentLogs(_) => {
                 // Go back to ProjectDetail if we have selected_project
                 if let Some(p) = &self.selected_project {
                     self.screen = Screen::ProjectDetail(p.id.clone());
                 } else {
                     self.screen = Screen::Projects;
                 }
                 self.message.clear();
            }
            _ => {}
        }
    }
}
