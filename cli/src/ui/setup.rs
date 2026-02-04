use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use tui_input::Input;

pub struct SetupState {
    pub server_url: Input,
    pub api_key: Input,
    pub focused_field: usize,
    pub error: Option<String>,
}

impl SetupState {
    pub fn new() -> Self {
        let server_url = Input::from("http://localhost:8080");

        Self {
            server_url,
            api_key: Input::default(),
            focused_field: 0,
            error: None,
        }
    }

    pub fn next_field(&mut self) {
        self.focused_field = (self.focused_field + 1) % 2;
    }

    pub fn previous_field(&mut self) {
        if self.focused_field == 0 {
            self.focused_field = 1;
        } else {
            self.focused_field -= 1;
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &SetupState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            "Clickploy CLI Setup",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Welcome! Let's configure your CLI to connect to Clickploy."),
    ])
    .block(Block::default().borders(Borders::ALL).title("Setup"))
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    // Server URL input
    let server_url_style = if state.focused_field == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let server_url_block = Block::default()
        .borders(Borders::ALL)
        .title("Server URL")
        .border_style(server_url_style);

    let server_url_input = Paragraph::new(state.server_url.value()).block(server_url_block);
    f.render_widget(server_url_input, chunks[1]);

    // API Key input
    let api_key_style = if state.focused_field == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let api_key_block = Block::default()
        .borders(Borders::ALL)
        .title("API Key")
        .border_style(api_key_style);

    let masked_key = if state.api_key.value().is_empty() {
        ""
    } else {
        "••••••••••••••••••••••••••••••••"
    };

    let api_key_input = Paragraph::new(masked_key).block(api_key_block);
    f.render_widget(api_key_input, chunks[2]);

    // Instructions
    let instructions = Paragraph::new(vec![
        Line::from(Span::styled(
            "How to get your API key:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("1. Open your browser and navigate to:"),
        Line::from(Span::styled(
            "   <server-url>/settings/session",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from("2. Copy your API key from the page"),
        Line::from("3. Paste it into the API Key field above"),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter on URL to open browser | Tab to switch | Esc to quit",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().borders(Borders::ALL).title("Instructions"))
    .wrap(Wrap { trim: true });
    f.render_widget(instructions, chunks[3]);

    // Error message
    if let Some(error) = &state.error {
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .wrap(Wrap { trim: true });
        f.render_widget(error_widget, chunks[4]);
    }
}
