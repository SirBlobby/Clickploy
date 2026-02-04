use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::ui::app::App;

pub fn render(f: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Header
    let header = Paragraph::new("Documentation & Help")
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Content
    let content_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Clickploy CLI Quick Reference", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  ↑↓ / j k   - Navigate lists"),
        Line::from("  Enter       - Select / View details"),
        Line::from("  Backspace   - Go back"),
        Line::from("  Tab         - Next field (forms)"),
        Line::from("  q / Ctrl+C  - Quit"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Main Screen Shortcuts:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  n - Create new project"),
        Line::from("  a - View activity"),
        Line::from("  d - View deployments"),
        Line::from("  w - Network overview"),
        Line::from("  t - Storage management"),
        Line::from("  s - Settings"),
        Line::from("  h - Help (this screen)"),
        Line::from("  r - Refresh current view"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Project Actions:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  r - Redeploy project"),
        Line::from("  l - View logs"),
        Line::from("  c - View settings"),
        Line::from(""),
        Line::from(vec![
            Span::styled("For full documentation, visit:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("http://localhost:8080/docs", Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
        ]),
    ];

    let content = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default())
        .wrap(Wrap { trim: false });
    f.render_widget(content, chunks[1]);

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
    f.render_widget(footer, chunks[2]);
}
