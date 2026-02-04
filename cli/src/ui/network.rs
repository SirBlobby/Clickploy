use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::ui::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Header
    let header = Paragraph::new("Network Overview")
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Projects with their network info
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

            let status_color = match status.as_str() {
                "live" => Color::Green,
                "building" => Color::Yellow,
                "failed" => Color::Red,
                _ => Color::DarkGray,
            };

            let symbol = match status.as_str() {
                "live" => "●",
                "building" => "◐",
                "failed" => "✗",
                _ => "○",
            };

            let display_text = format!(
                "{} {} - Port {} - http://localhost:{}",
                symbol, p.name, p.port, p.port
            );

            ListItem::new(display_text).style(Style::default().fg(status_color))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Active Services ({})", app.projects.len()))
        );

    f.render_widget(list, chunks[1]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Backspace", Style::default().fg(Color::Yellow)),
            Span::raw(" Back | "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(" Refresh | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
