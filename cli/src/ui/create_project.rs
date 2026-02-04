use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use tui_input::Input;

pub struct CreateProjectState {
    pub name: Input,
    pub repo_url: Input,
    pub build_command: Input,
    pub start_command: Input,
    pub install_command: Input,
    pub focused_field: usize,
    pub error: Option<String>,
}

impl CreateProjectState {
    pub fn new() -> Self {
        Self {
            name: Input::default(),
            repo_url: Input::default(),
            build_command: Input::default(),
            start_command: Input::default(),
            install_command: Input::default(),
            focused_field: 0,
            error: None,
        }
    }

    pub fn next_field(&mut self) {
        self.focused_field = (self.focused_field + 1) % 5;
    }

    pub fn previous_field(&mut self) {
        if self.focused_field == 0 {
            self.focused_field = 4;
        } else {
            self.focused_field -= 1;
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &CreateProjectState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Name
            Constraint::Length(3), // Repo
            Constraint::Length(3), // Install
            Constraint::Length(3), // Build
            Constraint::Length(3), // Start
            Constraint::Min(0),    // Error/Help
        ])
        .split(area);

    let title = Paragraph::new("Create New Project")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let fields = [
        ("Project Name", &state.name),
        ("Git Repository URL", &state.repo_url),
        ("Install Command (optional)", &state.install_command),
        ("Build Command (optional)", &state.build_command),
        ("Start Command (optional)", &state.start_command),
    ];

    for (i, (label, input)) in fields.iter().enumerate() {
        let style = if state.focused_field == i {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(*label)
            .border_style(style);

        let widget = Paragraph::new(input.value()).block(block);
        f.render_widget(widget, chunks[i + 1]);
    }

    if let Some(error) = &state.error {
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .wrap(Wrap { trim: true });
        f.render_widget(error_widget, chunks[6]);
    } else {
        let help = Paragraph::new("Tab: Next Field | Enter: Create | Esc: Cancel")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[6]);
    }
}
