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
                Constraint::Length(12), // Git Config
                Constraint::Length(10), // Build Settings
                Constraint::Length(6),  // Networking
                Constraint::Min(8),     // Env vars
                Constraint::Length(5),  // Webhook
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        // Git Configuration
        let git_info = vec![
            Line::from(vec![
                Span::styled("Git Configuration", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Project Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&project.name),
            ]),
            Line::from(vec![
                Span::styled("Repository URL: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&project.repo_url),
            ]),
            Line::from(vec![
                Span::styled("Git Token: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("••••••••"),
            ]),
        ];

        let git_widget = Paragraph::new(git_info)
            .block(Block::default().borders(Borders::ALL).title("Git Configuration"))
            .wrap(Wrap { trim: true });
        f.render_widget(git_widget, chunks[0]);

        // Build & Output Settings
        let build_info = vec![
            Line::from(vec![
                Span::styled("Build & Output Settings", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Runtime: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if project.runtime.is_empty() { "auto (nodejs)" } else { &project.runtime }),
            ]),
            Line::from(vec![
                Span::styled("Install Cmd: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if project.install_command.is_empty() { "default" } else { &project.install_command }),
            ]),
            Line::from(vec![
                Span::styled("Build Cmd: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if project.build_command.is_empty() { "default" } else { &project.build_command }),
            ]),
            Line::from(vec![
                Span::styled("Start Cmd: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if project.start_command.is_empty() { "default" } else { &project.start_command }),
            ]),
        ];

        let build_widget = Paragraph::new(build_info)
            .block(Block::default().borders(Borders::ALL).title("Build & Output"))
            .wrap(Wrap { trim: true });
        f.render_widget(build_widget, chunks[1]);

        // Networking
        let network_info = vec![
            Line::from(vec![
                Span::styled("Internal Port: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(project.port.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Local URL: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("http://localhost:{}", project.port),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
        ];

        let network_widget = Paragraph::new(network_info)
            .block(Block::default().borders(Borders::ALL).title("Networking"))
            .wrap(Wrap { trim: true });
        f.render_widget(network_widget, chunks[2]);

        // Environment Variables
        let env_items = if let Some(env_vars) = &project.env_vars {
            if env_vars.is_empty() {
                vec![ListItem::new("No environment variables configured")]
            } else {
                env_vars
                    .iter()
                    .map(|e| {
                        ListItem::new(format!("{} = ••••••••", e.key))
                            .style(Style::default().fg(Color::Green))
                    })
                    .collect()
            }
        } else {
            vec![ListItem::new("No environment variables configured")]
        };

        let env_list = List::new(env_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Environment Variables ({})", 
                        project.env_vars.as_ref().map(|e| e.len()).unwrap_or(0)
                    ))
            );
        f.render_widget(env_list, chunks[3]);

        // Webhook Integration
        let webhook_info = vec![
            Line::from(vec![
                Span::styled("Webhook URL: ", Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("http://localhost:8080/projects/{}/webhook/{}", 
                        project.id, project.webhook_secret),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
        ];

        let webhook_widget = Paragraph::new(webhook_info)
            .block(Block::default().borders(Borders::ALL).title("Webhook Integration"))
            .wrap(Wrap { trim: true });
        f.render_widget(webhook_widget, chunks[4]);

        // Footer
        let footer_text = vec![
            Line::from(vec![
                Span::styled("Backspace", Style::default().fg(Color::Yellow)),
                Span::raw(" Back | "),
                Span::styled("e", Style::default().fg(Color::Yellow)),
                Span::raw(" Edit (Web UI) | "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" Quit"),
            ]),
        ];

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(footer, chunks[5]);
    } else {
        let loading = Paragraph::new("Loading project settings...")
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(loading, area);
    }
}
