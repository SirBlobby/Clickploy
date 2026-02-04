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
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(4)])
        .split(area);

    // Header
    let header_text = if let Some(user) = &app.user {
        format!("Clickploy CLI - {} ({})", user.name, user.email)
    } else {
        "Clickploy CLI".to_string()
    };
    
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, chunks[0]);

    // Projects list
    let items: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(i, p)| {
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
                "live" => Color::Green,
                "building" => Color::Yellow,
                "failed" => Color::Red,
                _ => Color::White,
            };

            let symbol = match status.as_str() {
                "live" => "●",
                "building" => "◐",
                "failed" => "✗",
                _ => "○",
            };

            let display_name = format!(
                "{} {} - {} (port {})",
                symbol, p.name, status, p.port
            );

            let style = if i == app.selected_index {
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else {
                Style::default().fg(color)
            };

            ListItem::new(display_name).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Projects ({})", app.projects.len()))
        )
        .highlight_symbol(">> ");

    f.render_widget(list, chunks[1]);

    // Footer with controls
    let footer_text = vec![
        Line::from(vec![
            Span::styled("↑↓/jk", Style::default().fg(Color::Yellow)),
            Span::raw(" Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" Details | "),
            Span::styled("n", Style::default().fg(Color::Yellow)),
            Span::raw(" New | "),
            Span::styled("d", Style::default().fg(Color::Yellow)),
            Span::raw(" Deployments | "),
            Span::styled("w", Style::default().fg(Color::Yellow)),
            Span::raw(" Network"),
        ]),
        Line::from(vec![
            Span::styled("a", Style::default().fg(Color::Yellow)),
            Span::raw(" Activity | "),
            Span::styled("t", Style::default().fg(Color::Yellow)),
            Span::raw(" Storage | "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(" Help | "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(" Settings | "),
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

    // Show message or error
    if let Some(error) = &app.error {
        let error_area = Rect {
            x: area.width / 4,
            y: area.height / 2 - 2,
            width: area.width / 2,
            height: 5,
        };
        
        let error_widget = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_widget, error_area);
    }
}
