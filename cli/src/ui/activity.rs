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
    let header = Paragraph::new("Recent Deployment Activity")
        .block(Block::default().borders(Borders::ALL).title("Activity"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(header, chunks[0]);

    // Activity list
    let items: Vec<ListItem> = app
        .activity
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let status_color = match d.status.as_str() {
                "live" => Color::Green,
                "building" => Color::Yellow,
                "failed" => Color::Red,
                _ => Color::White,
            };

            let symbol = match d.status.as_str() {
                "live" => "●",
                "building" => "◐",
                "failed" => "✗",
                _ => "○",
            };

            let commit_short = if d.commit.len() > 7 {
                &d.commit[..7]
            } else {
                &d.commit
            };

            let timestamp = d.created_at.split('T').next().unwrap_or(&d.created_at);

            let display_text = format!(
                "{} {} - {} - {} - {}",
                symbol,
                d.project_id,
                d.status,
                commit_short,
                timestamp
            );

            let style = if i == app.selected_index {
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else {
                Style::default().fg(status_color)
            };

            ListItem::new(display_text).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Deployments ({})", app.activity.len()))
        )
        .highlight_symbol(">> ");

    f.render_widget(list, chunks[1]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" View Logs | "),
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

    // Show error if any
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
