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
    let header = Paragraph::new("All Deployments")
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Deployments list from activity
    let items: Vec<ListItem> = app
        .activity
        .iter()
        .map(|d| {
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

            let date = d.created_at.split('T').next().unwrap_or(&d.created_at);

            let display_text = format!(
                "{} {} - {} - {}",
                symbol, d.status, commit_short, date
            );

            ListItem::new(display_text).style(Style::default().fg(status_color))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Recent Deployments ({})", app.activity.len()))
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
