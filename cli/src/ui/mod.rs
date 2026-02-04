pub mod app;
pub mod setup;
pub mod projects;
pub mod project_detail;
pub mod project_settings;
pub mod activity;
pub mod settings;
pub mod create_project;
pub mod deployment_logs;
pub mod deployments;
pub mod network;
pub mod storage;
pub mod docs;

pub use app::{App, Screen};
pub use setup::SetupState;
pub use create_project::CreateProjectState;

#[derive(Debug, Clone)]
pub struct CreateDatabaseState {
    pub name: String,
    pub db_type: String,
    pub focused_field: usize,
}

impl CreateDatabaseState {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            db_type: "sqlite".to_string(),
            focused_field: 0,
        }
    }

    pub fn reset(&mut self) {
        self.name.clear();
        self.db_type = "sqlite".to_string();
        self.focused_field = 0;
    }
}
