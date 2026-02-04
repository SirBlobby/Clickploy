use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::ui::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    if let Some(project) = &app.selected_project {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Project info
        let info_text = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&project.name),
            ]),
            Line::from(vec![
                Span::styled("Repository: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&project.repo_url),
            ]),
            Line::from(vec![
                Span::styled("Port: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(project.port.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Runtime: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if project.runtime.is_empty() { "auto" } else { &project.runtime }),
            ]),
            Line::from(vec![
                Span::styled("URL: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("http://localhost:{}", project.port),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
        ];

        let info = Paragraph::new(info_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Project Details")
            )
            .wrap(Wrap { trim: true });
        f.render_widget(info, chunks[0]);

        // Deployments
        let deployments = if let Some(deps) = &project.deployments {
            deps.iter()
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

                    ListItem::new(format!(
                        "{} {} - {} - {}",
                        symbol,
                        d.status,
                        commit_short,
                        d.created_at.split('T').next().unwrap_or(&d.created_at)
                    ))
                    .style(Style::default().fg(status_color))
                })
                .collect::<Vec<_>>()
        } else {
            vec![ListItem::new("No deployments yet")]
        };

        let deployment_list = List::new(deployments)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Deployment History")
            );
        f.render_widget(deployment_list, chunks[1]);

        // Footer
        let footer_text = vec![
            Line::from(vec![
                Span::styled("Backspace", Style::default().fg(Color::Yellow)),
                Span::raw(" Back | "),
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(" Redeploy | "),
                Span::styled("s", Style::default().fg(Color::Yellow)),
                Span::raw(" Stop | "),
                Span::styled("l", Style::default().fg(Color::Yellow)),
                Span::raw(" View Logs | "),
                Span::styled("c", Style::default().fg(Color::Yellow)),
                Span::raw(" Settings | "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" Quit"),
            ]),
        ];

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(footer, chunks[2]);

        // Show message or error
        if !app.message.is_empty() && app.error.is_none() {
            let msg_area = Rect {
                x: area.width / 4,
                y: area.height / 2 - 2,
                width: area.width / 2,
                height: 5,
            };
            
            let msg_widget = Paragraph::new(app.message.as_str())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Info"));
            f.render_widget(msg_widget, msg_area);
        }

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
    } else {
        let loading = Paragraph::new("Loading project details...")
            .block(Block::default().borders(Borders::ALL).title("Project"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(loading, area);
    }
}
