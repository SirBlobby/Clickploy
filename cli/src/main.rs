use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use serde::Deserialize;
use std::{io, time::Duration};

#[derive(Debug, Deserialize, Clone)]
struct Project {
    #[serde(rename = "ID")]
    id: u32,
    name: String,
    repo_url: String,
    port: u32,
    deployments: Option<Vec<Deployment>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Deployment {
    #[serde(rename = "ID")]
    id: u32,
    status: String,
}

#[derive(Debug, Deserialize, Clone)]
struct ProjectCorrected {
    #[serde(rename = "ID")]
    id: u32,
    name: String,
    repo_url: String,
    port: u32,
    deployments: Option<Vec<Deployment>>,
}

struct App {
    projects: Vec<ProjectCorrected>,
    state: ListState,
    message: String,
}

impl App {
    fn new() -> App {
        let mut state = ListState::default();
        state.select(Some(0));
        App {
            projects: vec![],
            state,
            message: "Fetching...".to_string(),
        }
    }

    async fn fetch_data(&mut self) {
        match reqwest::get("http://localhost:8080/api/projects").await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<Vec<ProjectCorrected>>().await {
                        Ok(projects) => {
                            self.projects = projects;
                            self.message = format!("Loaded {} projects", self.projects.len());
                        }
                        Err(e) => self.message = format!("Parse error: {}", e),
                    }
                } else {
                     self.message = format!("Error: {}", resp.status());
                }
            }
            Err(e) => self.message = format!("Req error: {}", e),
        }
    }

    fn next(&mut self) {
        if self.projects.is_empty() {
             return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.projects.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.projects.is_empty() {
             return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.projects.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    
    app.fetch_data().await;

    let res = run_app(&mut terminal, app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => app.fetch_data().await,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    let header_text = format!("Clickploy CLI - {} (Press 'r' to refresh, 'q' to quit)", app.message);
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = app
        .projects
        .iter()
        .map(|p| {
            let status = if let Some(deps) = &p.deployments {
                if let Some(first) = deps.first() {
                    first.status.clone()
                } else {
                    "unknown".to_string()
                }
            } else {
                "unknown".to_string()
            };

            let color = match status.as_str() {
                "live" | "success" => Color::Green,
                "building" => Color::Yellow,
                "failed" => Color::Red,
                _ => Color::White,
            };
            
            let display_name = format!("{} - {}", p.name, status);
            ListItem::new(display_name).style(Style::default().fg(color))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Projects"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut app.state);
}
