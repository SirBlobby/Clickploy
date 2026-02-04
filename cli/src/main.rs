mod api;
mod config;
mod models;
mod ui;

use anyhow::Result;
use api::ApiClient;
use config::{config_exists, delete_config, load_config, save_config, Config};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};
use tokio::sync::mpsc;
use ui::{App, Screen, SetupState};

#[tokio::main]
async fn main() -> Result<()> {
    // Check if config exists
    if !config_exists() {
        // Run setup
        run_setup().await?;
    }

    // Load config
    let config = load_config()?;

    // Validate connection
    let client = ApiClient::new(config.server_url.clone(), config.api_key.clone());
    
    match client.validate_connection().await {
        Ok(user) => {
            // Run main app
            run_app(client, user).await?;
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            eprintln!("Please reconfigure by running the setup again.");
            delete_config()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn run_setup() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut setup_state = SetupState::new();
    let mut should_quit = false;

    loop {
        terminal.draw(|f| {
            ui::setup::render(f, f.area(), &setup_state);
        })?;

        if event::poll(Duration::from_millis(100))? {
                        if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        should_quit = true;
                        break;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        should_quit = true;
                        break;
                    }
                    KeyCode::Tab => {
                        setup_state.next_field();
                    }
                    KeyCode::BackTab => {
                        setup_state.previous_field();
                    }
                    KeyCode::Enter => {
                        let server_url = setup_state.server_url.value().trim().to_string();
                        let api_key = setup_state.api_key.value().trim().to_string();

                        if setup_state.focused_field == 0 {
                            if !server_url.is_empty() {
                                let url = format!("{}/settings/session", server_url.trim_end_matches('/'));
                                let _ = webbrowser::open(&url);
                                setup_state.next_field();
                            } else {
                                setup_state.error = Some("Server URL cannot be empty".to_string());
                            }
                            continue;
                        }

                        // Validate and save
                        if server_url.is_empty() {
                            setup_state.error = Some("Server URL cannot be empty".to_string());
                            continue;
                        }

                        if api_key.is_empty() {
                            setup_state.error = Some("API key cannot be empty".to_string());
                            continue;
                        }

                        // Test connection
                        let test_client = ApiClient::new(server_url.clone(), api_key.clone());
                        
                        match test_client.validate_connection().await {
                            Ok(_) => {
                                // Save config
                                let config = Config::new(server_url, api_key);
                                if let Err(e) = save_config(&config) {
                                    setup_state.error = Some(format!("Failed to save config: {}", e));
                                    continue;
                                }
                                break;
                            }
                            Err(e) => {
                                setup_state.error = Some(format!("Connection failed: {}", e));
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if setup_state.focused_field == 0 {
                            setup_state.server_url.handle(tui_input::InputRequest::InsertChar(c));
                        } else {
                            setup_state.api_key.handle(tui_input::InputRequest::InsertChar(c));
                        }
                    }
                    KeyCode::Backspace => {
                        if setup_state.focused_field == 0 {
                            setup_state.server_url.handle(tui_input::InputRequest::DeletePrevChar);
                        } else {
                            setup_state.api_key.handle(tui_input::InputRequest::DeletePrevChar);
                        }
                    }
                    _ => {}
                }
            } else if let Event::Mouse(mouse) = event::read()? {
                if mouse.kind == event::MouseEventKind::Down(crossterm::event::MouseButton::Left) {
                    // Simple hit testing based on known layout in setup.rs
                    // URL input is at chunks[1]
                    // API key input is at chunks[2]
                    // We need to know where these chunks are. 
                    // Since we can't easily share layout state, we'll approximate or toggle.
                    // Actually, a simpler way for now is just to support field switching via click 
                    // if we know the Y coordinates.
                    // Layout:
                    // Title: 7 lines
                    // URL: 3 lines (y: 7-9)
                    // API: 3 lines (y: 10-12)
                    
                    let y = mouse.row;
                    if y >= 7 && y < 10 {
                        setup_state.focused_field = 0;
                    } else if y >= 10 && y < 13 {
                        setup_state.focused_field = 1;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if should_quit {
        std::process::exit(0);
    }

    Ok(())
}

async fn run_app(client: ApiClient, user: models::User) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.screen = Screen::Projects;
    app.user = Some(user.clone());
    app.message = format!("Welcome, {}!", user.name);

    // Initial data fetch
    app.fetch_projects(&client).await?;

    let res = run_app_loop(&mut terminal, &mut app, &client).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn stream_logs(server_url: String, deployment_id: String, tx: mpsc::UnboundedSender<String>) -> Result<()> {
    use tokio_tungstenite::connect_async;
    use futures_util::StreamExt;

    let ws_url = server_url
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    let url = format!("{}/api/deployments/{}/logs/stream", ws_url, deployment_id);

    let (ws_stream, _) = connect_async(&url).await?;
    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    if tx.send(text.to_string()).is_err() {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}

async fn run_app_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    client: &ApiClient,
) -> Result<()> {
    let (log_tx, mut log_rx) = mpsc::unbounded_channel::<String>();
    let mut ws_task: Option<tokio::task::JoinHandle<()>> = None;
    let mut current_deployment_id: Option<String> = None;

    loop {
        // Check if we need to start/stop WebSocket connection
        if let Screen::DeploymentLogs(ref id) = app.screen {
            if current_deployment_id.as_ref() != Some(id) {
                // Stop old task
                if let Some(task) = ws_task.take() {
                    task.abort();
                }
                
                // Start new WebSocket connection
                let deployment_id = id.clone();
                let server_url = load_config().ok().map(|c| c.server_url).unwrap_or_else(|| "http://localhost:8080".to_string());
                let tx = log_tx.clone();
                
                ws_task = Some(tokio::spawn(async move {
                    if let Err(e) = stream_logs(server_url, deployment_id, tx).await {
                        eprintln!("WebSocket error: {}", e);
                    }
                }));
                
                current_deployment_id = Some(id.clone());
                app.live_logs.clear();
            }
        } else if ws_task.is_some() {
            // Stop WebSocket if we're not on logs screen
            if let Some(task) = ws_task.take() {
                task.abort();
            }
            current_deployment_id = None;
            app.live_logs.clear();
        }

        // Check for new log messages
        while let Ok(log_chunk) = log_rx.try_recv() {
            app.live_logs.push_str(&log_chunk);
            // Auto-scroll to bottom if at bottom
            if app.log_scroll >= u16::MAX - 10 {
                app.log_scroll = u16::MAX;
            }
        }

        terminal.draw(|f| {
            // Split layout for status bar
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Min(0),
                    ratatui::layout::Constraint::Length(1),
                ])
                .split(f.area());

            let content_area = chunks[0];
            let status_area = chunks[1];

            match &app.screen {
                Screen::Setup => {
                    // Should not reach here
                }
                Screen::Projects => {
                    ui::projects::render(f, content_area, app);
                }
                Screen::CreateProject => {
                    ui::create_project::render(f, content_area, &app.create_project_state);
                }
                Screen::DeploymentLogs(id) => {
                    ui::deployment_logs::render(f, content_area, app, id);
                }
                Screen::ProjectDetail(_id) => {
                    ui::project_detail::render(f, content_area, app);
                }
                Screen::ProjectSettings(_id) => {
                    ui::project_settings::render(f, content_area, app);
                }
                Screen::Deployments => {
                    ui::deployments::render(f, content_area, app);
                }
                Screen::Activity => {
                    ui::activity::render(f, content_area, app);
                }
                Screen::Network => {
                    ui::network::render(f, content_area, app);
                }
                Screen::Storage => {
                    ui::storage::render(f, content_area, app);
                }
                Screen::CreateDatabase => {
                    ui::storage::render_create_database(f, content_area, app);
                }
                Screen::Docs => {
                    ui::docs::render(f, content_area, app);
                }
                Screen::Settings => {
                    if let Ok(config) = load_config() {
                        ui::settings::render(f, content_area, &config);
                    }
                }
            }

            // Render Status Bar
            let status_text = format!(
                " Clickploy CLI | User: {} | Screen: {:?} | q: Quit", 
                app.user.as_ref().map(|u| u.name.as_str()).unwrap_or("Unknown"),
                app.screen
            );
            let status_bar = ratatui::widgets::Paragraph::new(status_text)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Black).bg(ratatui::style::Color::Cyan));
            f.render_widget(status_bar, status_area);

        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Global keys
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true;
                        return Ok(());
                    }
                    _ => {}
                }

                // Screen-specific keys
                match app.screen.clone() {
                    Screen::Projects => {
                        match key.code {
                            KeyCode::Down | KeyCode::Char('j') => app.next(),
                            KeyCode::Up | KeyCode::Char('k') => app.previous(),
                            KeyCode::Enter => {
                                app.select_project();
                                if let Screen::ProjectDetail(id) = &app.screen.clone() {
                                    app.fetch_project_detail(client, id).await?;
                                }
                            }
                            KeyCode::Char('n') => {
                                app.screen = Screen::CreateProject;
                                app.create_project_state = ui::CreateProjectState::new();
                            }
                            KeyCode::Char('r') => {
                                app.fetch_projects(client).await?;
                            }
                            KeyCode::Char('a') => {
                                app.screen = Screen::Activity;
                                app.selected_index = 0;
                                app.fetch_activity(client).await?;
                            }
                            KeyCode::Char('d') => {
                                app.screen = Screen::Deployments;
                                app.selected_index = 0;
                                app.fetch_activity(client).await?;
                            }
                            KeyCode::Char('w') => {
                                app.screen = Screen::Network;
                            }
                            KeyCode::Char('t') => {
                                app.screen = Screen::Storage;
                                app.selected_index = 0;
                                app.fetch_storage_data(client).await?;
                            }
                            KeyCode::Char('h') => {
                                app.screen = Screen::Docs;
                            }
                            KeyCode::Char('s') => {
                                app.screen = Screen::Settings;
                            }
                            _ => {}
                        }
                    }
                    Screen::CreateProject => {
                        match key.code {
                            KeyCode::Esc => {
                                app.screen = Screen::Projects;
                            }
                            KeyCode::Tab => {
                                app.create_project_state.next_field();
                            }
                            KeyCode::BackTab => {
                                app.create_project_state.previous_field();
                            }
                            KeyCode::Enter => {
                                // Create project
                                let name = app.create_project_state.name.value().trim().to_string();
                                let repo = app.create_project_state.repo_url.value().trim().to_string();
                                
                                if name.is_empty() || repo.is_empty() {
                                    app.create_project_state.error = Some("Name and Repo URL are required".to_string());
                                    continue;
                                }

                                let build_cmd = app.create_project_state.build_command.value().trim();
                                let start_cmd = app.create_project_state.start_command.value().trim();
                                let install_cmd = app.create_project_state.install_command.value().trim();

                                let request = models::CreateProjectRequest {
                                    name,
                                    repo,
                                    port: Some(3000), // Default port
                                    git_token: None,
                                    env_vars: None,
                                    build_command: if build_cmd.is_empty() { None } else { Some(build_cmd.to_string()) },
                                    start_command: if start_cmd.is_empty() { None } else { Some(start_cmd.to_string()) },
                                    install_command: if install_cmd.is_empty() { None } else { Some(install_cmd.to_string()) },
                                    runtime: None,
                                };

                                app.message = "Creating project...".to_string();
                                match client.create_project(request).await {
                                    Ok(_) => {
                                        app.message = "Project created successfully".to_string();
                                        app.screen = Screen::Projects;
                                        app.fetch_projects(client).await?;
                                    }
                                    Err(e) => {
                                        app.create_project_state.error = Some(format!("Failed to create project: {}", e));
                                    }
                                }
                            }
                            KeyCode::Char(c) => {
                                match app.create_project_state.focused_field {
                                    0 => app.create_project_state.name.handle(tui_input::InputRequest::InsertChar(c)),
                                    1 => app.create_project_state.repo_url.handle(tui_input::InputRequest::InsertChar(c)),
                                    2 => app.create_project_state.install_command.handle(tui_input::InputRequest::InsertChar(c)),
                                    3 => app.create_project_state.build_command.handle(tui_input::InputRequest::InsertChar(c)),
                                    4 => app.create_project_state.start_command.handle(tui_input::InputRequest::InsertChar(c)),
                                    _ => None,
                                };
                            }
                            KeyCode::Backspace => {
                                match app.create_project_state.focused_field {
                                    0 => app.create_project_state.name.handle(tui_input::InputRequest::DeletePrevChar),
                                    1 => app.create_project_state.repo_url.handle(tui_input::InputRequest::DeletePrevChar),
                                    2 => app.create_project_state.install_command.handle(tui_input::InputRequest::DeletePrevChar),
                                    3 => app.create_project_state.build_command.handle(tui_input::InputRequest::DeletePrevChar),
                                    4 => app.create_project_state.start_command.handle(tui_input::InputRequest::DeletePrevChar),
                                    _ => None,
                                };
                            }
                            _ => {}
                        }
                    }
                    Screen::ProjectDetail(id) => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            KeyCode::Char('r') => {
                                // Redeploy
                                app.message = "Redeploying...".to_string();
                                match client.redeploy_project(&id, None).await {
                                    Ok(_) => {
                                        app.message = "Redeploy started successfully".to_string();
                                        app.error = None;
                                        // Refresh project details
                                        tokio::time::sleep(Duration::from_millis(500)).await;
                                        app.fetch_project_detail(client, &id).await?;
                                    }
                                    Err(e) => {
                                        app.error = Some(format!("Redeploy failed: {}", e));
                                    }
                                }
                            }
                            KeyCode::Char('s') => {
                                // Stop project
                                app.message = "Stopping project...".to_string();
                                match client.stop_project(&id).await {
                                    Ok(_) => {
                                        app.message = "Project stopped successfully".to_string();
                                        app.error = None;
                                        // Refresh project details
                                        tokio::time::sleep(Duration::from_millis(500)).await;
                                        app.fetch_project_detail(client, &id).await?;
                                    }
                                    Err(e) => {
                                        app.error = Some(format!("Stop failed: {}", e));
                                    }
                                }
                            }
                            KeyCode::Char('l') => {
                                // View Logs for latest deployment
                                if let Some(project) = &app.selected_project {
                                    if let Some(deployments) = &project.deployments {
                                        if let Some(latest) = deployments.first() {
                                            app.screen = Screen::DeploymentLogs(latest.id.clone());
                                        } else {
                                            app.error = Some("No deployments found".to_string());
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('c') => {
                                // View project settings
                                app.screen = Screen::ProjectSettings(id.clone());
                            }
                            _ => {}
                        }
                    }
                    Screen::ProjectSettings(_id) => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            _ => {}
                        }
                    }
                    Screen::Deployments => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            KeyCode::Char('r') => {
                                app.fetch_activity(client).await?;
                            }
                            _ => {}
                        }
                    }
                    Screen::Network => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            KeyCode::Char('r') => {
                                app.fetch_projects(client).await?;
                            }
                            _ => {}
                        }
                    }
                    Screen::Storage => {
                        match key.code {
                            KeyCode::Backspace | KeyCode::Esc => {
                                if app.show_db_credentials {
                                    app.show_db_credentials = false;
                                    app.db_credentials = None;
                                } else {
                                    app.go_back();
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if !app.show_db_credentials {
                                    app.previous();
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                if !app.show_db_credentials {
                                    app.next();
                                }
                            }
                            KeyCode::Enter => {
                                if !app.show_db_credentials && !app.databases.is_empty() {
                                    app.select_database();
                                    if let Some(db) = &app.selected_database {
                                        if db.db_type == "mongodb" {
                                            // Fetch credentials
                                            match client.get_database_credentials(db.id).await {
                                                Ok(creds) => {
                                                    app.db_credentials = Some(creds);
                                                }
                                                Err(e) => {
                                                    app.error = Some(format!("Failed to fetch credentials: {}", e));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('n') => {
                                if !app.show_db_credentials {
                                    app.create_database_state.reset();
                                    app.screen = Screen::CreateDatabase;
                                }
                            }
                            KeyCode::Char('d') => {
                                if !app.show_db_credentials && !app.databases.is_empty() && app.selected_index < app.databases.len() {
                                    let db_id = app.databases[app.selected_index].id;
                                    match client.delete_database(db_id).await {
                                        Ok(_) => {
                                            app.message = "Database deleted".to_string();
                                            let _ = app.fetch_storage_data(client).await;
                                        }
                                        Err(e) => {
                                            app.error = Some(format!("Failed to delete database: {}", e));
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('s') => {
                                if !app.show_db_credentials && !app.databases.is_empty() && app.selected_index < app.databases.len() {
                                    let db_id = app.databases[app.selected_index].id;
                                    match client.stop_database(db_id).await {
                                        Ok(_) => {
                                            app.message = "Database stopped".to_string();
                                            let _ = app.fetch_storage_data(client).await;
                                        }
                                        Err(e) => {
                                            app.error = Some(format!("Failed to stop database: {}", e));
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('r') => {
                                if !app.show_db_credentials && !app.databases.is_empty() && app.selected_index < app.databases.len() {
                                    let db_id = app.databases[app.selected_index].id;
                                    match client.restart_database(db_id).await {
                                        Ok(_) => {
                                            app.message = "Database restarted".to_string();
                                            let _ = app.fetch_storage_data(client).await;
                                        }
                                        Err(e) => {
                                            app.error = Some(format!("Failed to restart database: {}", e));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Screen::CreateDatabase => {
                        match key.code {
                            KeyCode::Esc => {
                                app.create_database_state.reset();
                                app.screen = Screen::Storage;
                            }
                            KeyCode::Tab => {
                                app.create_database_state.focused_field = 
                                    (app.create_database_state.focused_field + 1) % 2;
                            }
                            KeyCode::Char(c) => {
                                if app.create_database_state.focused_field == 0 {
                                    app.create_database_state.name.push(c);
                                }
                            }
                            KeyCode::Backspace => {
                                if app.create_database_state.focused_field == 0 {
                                    app.create_database_state.name.pop();
                                } else {
                                    app.create_database_state.reset();
                                    app.screen = Screen::Storage;
                                }
                            }
                            KeyCode::Up => {
                                if app.create_database_state.focused_field == 1 {
                                    app.create_database_state.db_type = if app.create_database_state.db_type == "mongodb" {
                                        "sqlite".to_string()
                                    } else {
                                        "mongodb".to_string()
                                    };
                                }
                            }
                            KeyCode::Down => {
                                if app.create_database_state.focused_field == 1 {
                                    app.create_database_state.db_type = if app.create_database_state.db_type == "sqlite" {
                                        "mongodb".to_string()
                                    } else {
                                        "sqlite".to_string()
                                    };
                                }
                            }
                            KeyCode::Enter => {
                                if !app.create_database_state.name.is_empty() {
                                    let name = app.create_database_state.name.clone();
                                    let db_type = app.create_database_state.db_type.clone();
                                    
                                    match client.create_database(name, db_type).await {
                                        Ok(_) => {
                                            app.message = "Database created successfully".to_string();
                                            app.create_database_state.reset();
                                            app.screen = Screen::Storage;
                                            let _ = app.fetch_storage_data(client).await;
                                        }
                                        Err(e) => {
                                            app.error = Some(format!("Failed to create database: {}", e));
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Screen::Docs => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            _ => {}
                        }
                    }
                    Screen::DeploymentLogs(_) => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.live_logs.clear();
                                app.log_scroll = 0;
                                app.go_back();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.log_scroll = app.log_scroll.saturating_add(1);
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.log_scroll = app.log_scroll.saturating_sub(1);
                            }
                            KeyCode::Home => {
                                app.log_scroll = 0;
                            }
                            KeyCode::End => {
                                app.log_scroll = u16::MAX;
                            }
                            KeyCode::PageDown => {
                                app.log_scroll = app.log_scroll.saturating_add(10);
                            }
                            KeyCode::PageUp => {
                                app.log_scroll = app.log_scroll.saturating_sub(10);
                            }
                            KeyCode::Char('q') => {
                                app.go_back();
                            }
                            _ => {}
                        }
                    }
                    Screen::Activity => {
                        match key.code {
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            KeyCode::Char('r') => {
                                app.fetch_activity(client).await?;
                            }
                            _ => {}
                        }
                    }
                    Screen::Settings => {
                        match key.code {
                            KeyCode::Backspace => {
                                app.go_back();
                            }
                            KeyCode::Char('c') => {
                                // Reconfigure
                                delete_config()?;
                                app.should_quit = true;
                                return Ok(());
                            }
                            KeyCode::Char('d') => {
                                // Delete config and quit
                                delete_config()?;
                                app.should_quit = true;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else if let Event::Mouse(_mouse) = event::read()? {
                // Handle mouse events if needed, for now just ignore to prevent errors
            }
        }
    }
}
