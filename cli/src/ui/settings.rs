use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::config::Config;

pub fn render(f: &mut Frame, area: Rect, config: &Config) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("CLI Configuration")
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, chunks[0]);

    // Config display
    let masked_key = if config.api_key.len() > 10 {
        format!("{}...{}", &config.api_key[..4], &config.api_key[config.api_key.len()-4..])
    } else {
        "••••••••".to_string()
    };

    let config_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Server URL: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&config.server_url, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("API Key: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(masked_key, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Config File: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("~/.config/clickploy/config.toml"),
        ]),
    ];

    let config_display = Paragraph::new(config_text)
        .block(Block::default().borders(Borders::ALL).title("Current Configuration"))
        .wrap(Wrap { trim: true });
    f.render_widget(config_display, chunks[1]);

    // Actions
    let actions_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Available Actions:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("c", Style::default().fg(Color::Yellow)),
            Span::raw(" - Reconfigure (change server URL and API key)"),
        ]),
        Line::from(vec![
            Span::styled("d", Style::default().fg(Color::Red)),
            Span::raw(" - Delete configuration (logout)"),
        ]),
    ];

    let actions = Paragraph::new(actions_text)
        .block(Block::default().borders(Borders::ALL).title("Actions"))
        .wrap(Wrap { trim: true });
    f.render_widget(actions, chunks[2]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Backspace", Style::default().fg(Color::Yellow)),
            Span::raw(" Back | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[3]);
}
