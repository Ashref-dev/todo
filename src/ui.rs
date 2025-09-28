use crate::app::{App, AppMode};
use crate::task::Priority;
use chrono::prelude::*;
use ratatui::{

    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let theme = app.theme_manager.get_current_theme();

    // Create a global background
    let background = Block::default().style(Style::default().bg(theme.background));
    f.render_widget(background, f.size());

    // Create a centered viewport based on the margin/zoom level
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(app.margin), // Top padding
                Constraint::Min(0),             // Content
                Constraint::Length(app.margin), // Bottom padding
            ]
            .as_ref(),
        )
        .split(f.size());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(app.margin * 2), // Left padding (x2 for better aspect ratio)
                Constraint::Min(0),                 // Content
                Constraint::Length(app.margin * 2), // Right padding
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);

    let viewport = horizontal_chunks[1];

    // Render the application within the calculated viewport
    let app_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(viewport);

    render_tasks(f, app, app_chunks[0]);
    render_footer(f, app, app_chunks[1]);

    match app.mode {
        AppMode::Insert => render_input_popup(f, app),
        AppMode::DateInput => render_date_input_popup(f, app),
        AppMode::Search => render_search_popup(f, app),
        AppMode::Confirm => render_confirm_dialog(f, app),
        AppMode::Help => render_help_dialog(f, app),
        _ => {}
    }
}

fn render_tasks(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = app.theme_manager.get_current_theme();
    let mut items = Vec::new();
    let displayed_tasks = app.get_displayed_tasks();

    for task in displayed_tasks.iter() {
        let (style, symbol) = if task.completed {
            (
                Style::default()
                    .fg(theme.surface2)
                    .add_modifier(Modifier::CROSSED_OUT),
                " ✔ ",
            )
        } else {
            (Style::default().fg(theme.text), " ❯ ")
        };
        let priority_style = Style::default().fg(match task.priority {
            Priority::High => theme.red,
            Priority::Medium => theme.yellow,
            Priority::Low => theme.green,
        });
        let priority_symbol = match task.priority {
            Priority::High => " ▲",
            Priority::Medium => " ●",
            Priority::Low => " ▼",
        };

        let mut spans = vec![
            Span::styled(symbol, Style::default().fg(theme.primary)),
            Span::raw(task.description.clone()),
            Span::styled(priority_symbol, priority_style),
        ];

        if let Some(due_date) = &task.due_date {
            let due_date_style = if Local::now().format("%Y-%m-%d").to_string() > *due_date {
                Style::default().fg(theme.red)
            } else {
                Style::default().fg(theme.subtext)
            };
            spans.push(Span::styled(
                format!(" (due: {})", due_date),
                due_date_style,
            ));
        }

        if !task.tags.is_empty() {
            spans.push(Span::raw(" "));
            for tag in task.tags.iter() {
                spans.push(Span::styled(tag, Style::default().fg(theme.accent)));
                spans.push(Span::raw(" "));
            }
        }

        items.push(ListItem::new(Line::from(spans)).style(style));

        // Only show subtasks if not in focus mode or if the parent task is not completed
        if !app.focus_mode || !task.completed {
            for sub_task in task.sub_tasks.iter() {
                // In focus mode, skip completed subtasks
                if app.focus_mode && sub_task.completed {
                    continue;
                }

                let (style, symbol) = if sub_task.completed {
                    (
                        Style::default()
                            .fg(theme.surface2)
                            .add_modifier(Modifier::CROSSED_OUT),
                        " ✔ ",
                    )
                } else {
                    (Style::default().fg(theme.text), " ❯ ")
                };
                let priority_style = Style::default().fg(match sub_task.priority {
                    Priority::High => theme.red,
                    Priority::Medium => theme.yellow,
                    Priority::Low => theme.green,
                });
                let priority_symbol = match sub_task.priority {
                    Priority::High => " ▲",
                    Priority::Medium => " ●",
                    Priority::Low => " ▼",
                };

                let mut spans = vec![
                    Span::raw("  ↳ "),
                    Span::styled(symbol, Style::default().fg(theme.primary)),
                    Span::raw(sub_task.description.clone()),
                    Span::styled(priority_symbol, priority_style),
                ];

                if let Some(due_date) = &sub_task.due_date {
                    let due_date_style = if Local::now().format("%Y-%m-%d").to_string() > *due_date
                    {
                        Style::default().fg(theme.red)
                    } else {
                        Style::default().fg(theme.subtext)
                    };
                    spans.push(Span::styled(
                        format!(" (due: {})", due_date),
                        due_date_style,
                    ));
                }

                if !sub_task.tags.is_empty() {
                    spans.push(Span::raw(" "));
                    for tag in sub_task.tags.iter() {
                        spans.push(Span::styled(tag, Style::default().fg(theme.accent)));
                        spans.push(Span::raw(" "));
                    }
                }

                items.push(ListItem::new(Line::from(spans)).style(style));
            }
        }
    }

    let title = match app.mode {
        AppMode::Search if !app.search_input.is_empty() => {
            let focus_indicator = if app.focus_mode { " [Focus]" } else { "" };
            format!(" To-Do (Search: {}){} ", app.search_input, focus_indicator)
        }
        AppMode::Search => {
            let focus_indicator = if app.focus_mode { " [Focus]" } else { "" };
            format!(" To-Do (Search Mode){} ", focus_indicator)
        }
        _ => {
            let focus_indicator = if app.focus_mode { " [Focus]" } else { "" };
            format!(" To-Do{} ", focus_indicator)
        }
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(theme.surface1))
                .title_style(Style::default().fg(theme.secondary)),
        )
        .highlight_style(
            Style::default()
                .bg(theme.surface0)
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" ➤ ");

    f.render_stateful_widget(list, area, &mut app.state);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme_manager.get_current_theme();
    let key_style = Style::default()
        .fg(theme.primary)
        .add_modifier(Modifier::BOLD);
    let desc_style = Style::default().fg(theme.subtext);

    macro_rules! key {
        ($key:expr, $desc:expr) => {
            vec![
                Span::styled($key, key_style),
                Span::styled($desc, desc_style),
            ]
        };
    }

    let help_spans = Line::from(
        key!("q", ":quit ")
            .into_iter()
            .chain(key!("h", ":help "))
            .chain(key!("a", ":add "))
            .chain(key!("d", ":delete "))
            .chain(key!("/", ":search "))
            .chain(key!("f", ":focus "))
            .chain(key!("t", ":theme "))
            .chain(key!("+", ":zoom "))
            .collect::<Vec<_>>(),
    );

    let help = Paragraph::new(help_spans).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.surface1))
            .title(" Controls ")
            .title_style(Style::default().fg(theme.secondary)),
    );

    f.render_widget(help, area);
}

fn render_input_popup(f: &mut Frame, app: &App) {
    let theme = app.theme_manager.get_current_theme();
    let area = centered_rect(60, 20, f.size());

    let title = if app.adding_subtask {
        " New Subtask "
    } else {
        " New Task "
    };

    let input_block = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.primary))
                .title_style(Style::default().fg(theme.secondary)),
        )
        .style(Style::default().fg(theme.text));

    f.render_widget(Clear, area);
    f.render_widget(input_block, area);
}

fn render_date_input_popup(f: &mut Frame, app: &App) {
    let theme = app.theme_manager.get_current_theme();
    let area = centered_rect(60, 20, f.size());
    let input_block = Paragraph::new(app.date_input.as_str())
        .block(
            Block::default()
                .title(" Set Due Date ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.primary))
                .title_style(Style::default().fg(theme.secondary)),
        )
        .style(Style::default().fg(theme.text));

    f.render_widget(Clear, area);
    f.render_widget(input_block, area);
}

fn render_search_popup(f: &mut Frame, app: &App) {
    let theme = app.theme_manager.get_current_theme();
    let area = centered_rect(80, 20, f.size());
    let search_help = "Search by: description, tags, priority (high/medium/low), status (completed/incomplete), due date";
    let input_text = format!("{}\n\n{}", app.search_input, search_help);

    let input_block = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(" Search Tasks ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.primary))
                .title_style(Style::default().fg(theme.secondary)),
        )
        .style(Style::default().fg(theme.text));

    f.render_widget(Clear, area);
    f.render_widget(input_block, area);
}

fn render_confirm_dialog(f: &mut Frame, app: &App) {
    let theme = app.theme_manager.get_current_theme();

    if let Some(dialog) = &app.confirm_dialog {
        let area = centered_rect(60, 30, f.size());
        let content = format!("{}\n\nPress 'y' to confirm, 'n' to cancel", dialog.message);

        let confirm_block = Paragraph::new(content)
            .block(
                Block::default()
                    .title(" Confirm Action ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.red))
                    .title_style(Style::default().fg(theme.red)),
            )
            .style(Style::default().fg(theme.text));

        f.render_widget(Clear, area);
        f.render_widget(confirm_block, area);
    }
}

fn render_help_dialog(f: &mut Frame, app: &App) {
    let theme = app.theme_manager.get_current_theme();
    let area = centered_rect(80, 80, f.size());

    let current_theme_name = &app.theme_manager.get_current_theme().name;
    let available_themes: Vec<String> = app.get_available_theme_names();

    let help_content = format!(
        "📝 Advanced Todo List - Help

🔑 KEYBINDINGS:
  Navigation:
    ↑/↓      - Move selection up/down
    Enter    - Toggle task completion
    
  Task Management:
    a        - Add new task
    s        - Add subtask to selected task
    d        - Delete selected task (with confirmation)
    p        - Cycle task priority (High/Medium/Low)
    D        - Set due date for selected task
    
  View & Search:
    /        - Search/filter tasks
    f        - Toggle focus mode (hide completed)
    +/-      - Zoom in/out
    
  Themes & Help:
    t        - Cycle through themes
    h/F1     - Show/hide this help
    C        - Clear completed tasks (with confirmation)
    q        - Quit application

🎨 THEMES:
  Current: {}
  Available: {}
  
🏷️  FEATURES:
  • Smart date parsing (\"tomorrow at 2pm\", \"monday\", etc.)
  • Tag support (#work #urgent)
  • Priority levels with visual indicators
  • Subtasks with nesting
  • Search by content, tags, priority, or status
  • Focus mode to hide completed tasks
  • Multiple color themes

💡 TIPS:
  • Use natural language for dates: \"call mom tomorrow at 3pm\"
  • Add tags with #: \"buy milk #shopping #urgent\"
  • Use focus mode (f) to concentrate on pending tasks
  • Cycle themes (t) to find your preferred style

Press ESC, h, F1, or q to close this help.",
        current_theme_name,
        available_themes.join(", ")
    );

    let help_block = Paragraph::new(help_content)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.primary))
                .title_style(Style::default().fg(theme.secondary)),
        )
        .style(Style::default().fg(theme.text))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(help_block, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
