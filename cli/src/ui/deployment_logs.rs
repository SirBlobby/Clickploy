use crate::ui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &App, deployment_id: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Logs
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Header with deployment info
    let deployment = if let Some(project) = &app.selected_project {
        if let Some(deployments) = &project.deployments {
            deployments.iter().find(|d| d.id == deployment_id)
        } else {
            None
        }
    } else {
        app.activity.iter().find(|d| d.id == deployment_id)
    };

    let header_text = if let Some(dep) = deployment {
        let commit_short = if dep.commit.len() > 7 {
            &dep.commit[..7]
        } else {
            &dep.commit
        };

        format!("Deployment: {} | Status: {} | Commit: {}", 
            deployment_id, dep.status, commit_short)
    } else {
        format!("Deployment: {}", deployment_id)
    };

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Live Logs"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Logs content - use live_logs if available, otherwise fall back to stored logs
    let logs_content = if !app.live_logs.is_empty() {
        app.live_logs.clone()
    } else if let Some(dep) = deployment {
        if dep.logs.is_empty() {
            "Connecting to log stream...".to_string()
        } else {
            dep.logs.clone()
        }
    } else {
        "Loading logs...".to_string()
    };

    let logs_widget = Paragraph::new(logs_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if !app.live_logs.is_empty() { "● Live Stream" } else { "Logs" }),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.log_scroll, 0));

    f.render_widget(logs_widget, chunks[1]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Backspace", Style::default().fg(Color::Yellow)),
            Span::raw(" Back | "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Scroll | "),
            Span::styled("Home/End", Style::default().fg(Color::Yellow)),
            Span::raw(" Top/Bottom | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
