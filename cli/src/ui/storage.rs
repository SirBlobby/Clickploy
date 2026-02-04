use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem, Gauge},
    Frame,
};
use crate::ui::app::App;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    if app.show_db_credentials {
        render_db_credentials(f, area, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("Storage Management")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Storage stats
    let storage_text = if let Some(stats) = &app.storage_stats {
        let used_gb = stats.used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_gb = stats.total as f64 / 1024.0 / 1024.0 / 1024.0;
        vec![
            Line::from(vec![
                Span::styled("Storage: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:.2} GB / {:.2} GB", used_gb, total_gb)),
            ]),
        ]
    } else {
        vec![Line::from("Loading storage stats...")]
    };

    let storage_block = Paragraph::new(storage_text)
        .block(Block::default().borders(Borders::ALL).title("Disk Usage"));
    f.render_widget(storage_block, chunks[1]);

    // Render gauge inside the storage block
    if let Some(stats) = &app.storage_stats {
        let gauge_area = Rect {
            x: chunks[1].x + 2,
            y: chunks[1].y + 2,
            width: chunks[1].width - 4,
            height: 2,
        };
        
        let gauge = Gauge::default()
            .block(Block::default())
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(stats.percent as u16);
        f.render_widget(gauge, gauge_area);
    }

    // Database list
    if app.databases.is_empty() {
        let empty_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("No databases yet", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Press 'n' to create a new database"),
        ];
        let empty = Paragraph::new(empty_text)
            .block(Block::default().borders(Borders::ALL).title("Databases"))
            .style(Style::default());
        f.render_widget(empty, chunks[2]);
    } else {
        let items: Vec<ListItem> = app.databases.iter().enumerate().map(|(i, db)| {
            let status_color = match db.status.as_str() {
                "running" => Color::Green,
                "stopped" => Color::Yellow,
                _ => Color::Red,
            };

            let content = vec![
                Line::from(vec![
                    Span::styled(&db.name, Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" - "),
                    Span::styled(&db.db_type, Style::default().fg(Color::Cyan)),
                    Span::raw(" - "),
                    Span::styled(&db.status, Style::default().fg(status_color)),
                    Span::raw(format!(" - Port: {}", db.port)),
                ]),
            ];

            let style = if i == app.selected_index {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Databases"));
        f.render_widget(list, chunks[2]);
    }

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" View Credentials | "),
            Span::styled("n", Style::default().fg(Color::Yellow)),
            Span::raw(" New Database | "),
            Span::styled("d", Style::default().fg(Color::Yellow)),
            Span::raw(" Delete | "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(" Stop | "),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(" Restart | "),
            Span::styled("Backspace", Style::default().fg(Color::Yellow)),
            Span::raw(" Back | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

fn render_db_credentials(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("Database Credentials")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Credentials
    if let Some(db) = &app.selected_database {
        let content = if let Some(creds) = &app.db_credentials {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("Database: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&db.name),
                ]),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&db.db_type),
                ]),
                Line::from(vec![
                    Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(&db.status, Style::default().fg(if db.status == "running" { Color::Green } else { Color::Yellow })),
                ]),
                Line::from(vec![
                    Span::styled("Port: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!("{}", db.port)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Username: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                    Span::raw(&creds.username),
                ]),
                Line::from(vec![
                    Span::styled("Password: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                    Span::raw(&creds.password),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Local URI:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
                ]),
                Line::from(creds.uri.clone()),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Public URI:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
                ]),
                Line::from(creds.public_uri.clone()),
            ]
        } else if db.db_type == "sqlite" {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("Database: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&db.name),
                ]),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&db.db_type),
                ]),
                Line::from(""),
                Line::from("SQLite databases are file-based and don't have credentials."),
                Line::from(format!("Database file: data/user_dbs/{}.db", &db.name)),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from("Loading credentials..."),
            ]
        };

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(paragraph, chunks[1]);
    }

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Backspace/Esc", Style::default().fg(Color::Yellow)),
            Span::raw(" Back | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" Quit"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

pub fn render_create_database(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("Create New Database")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Form
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(chunks[1]);

    // Name input
    let name_style = if app.create_database_state.focused_field == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let name_input = Paragraph::new(app.create_database_state.name.as_str())
        .block(Block::default().borders(Borders::ALL).title("Name").border_style(name_style));
    f.render_widget(name_input, form_chunks[0]);

    // Type selection
    let type_style = if app.create_database_state.focused_field == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    
    let db_types = vec!["sqlite", "mongodb"];
    let type_list: Vec<Line> = db_types.iter().map(|&t| {
        if t == app.create_database_state.db_type {
            Line::from(vec![
                Span::styled("▸ ", Style::default().fg(Color::Green)),
                Span::styled(t, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from(vec![
                Span::raw("  "),
                Span::raw(t),
            ])
        }
    }).collect();

    let type_select = Paragraph::new(type_list)
        .block(Block::default().borders(Borders::ALL).title("Type").border_style(type_style));
    f.render_widget(type_select, form_chunks[1]);

    // Footer
    let footer_text = vec![
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::raw(" Switch Field | "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(" Select Type | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" Create | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(" Cancel"),
        ]),
    ];

    let footer = Paragraph::new(footer_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
