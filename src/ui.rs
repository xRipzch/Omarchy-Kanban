use crate::app::{App, InputMode};
use crate::board::Column;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// render UI
pub fn draw(f: &mut Frame, app: &App) {
    // check if we're in a special view mode
    match app.input_mode {
        InputMode::ViewingTask | InputMode::EditingDescription => {
            draw_task_detail(f, app);
            return;
        }
        InputMode::ViewingHelp => {
            draw_help(f, app);
            return;
        }
        _ => {}
    }

    // make two workspaces: main area and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // Main area
            Constraint::Length(3),   // Footer
        ])
        .split(f.area());

    // drwa the three columns
    draw_columns(f, app, chunks[0]);

    // footer with help text or input field
    draw_footer(f, app, chunks[1]);
}

// draw the three columns
fn draw_columns(f: &mut Frame, app: &App, area: Rect) {
    // split main area into three equal columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    // draw each column
    draw_column(f, app, Column::Todo, columns[0]);
    draw_column(f, app, Column::InProgress, columns[1]);
    draw_column(f, app, Column::Done, columns[2]);
}

/// draw single column with task cards
fn draw_column(f: &mut Frame, app: &App, column: Column, area: Rect) {
    let tasks = app.board.get_column(column);
    let is_selected_column = app.selected_column == column;

    // highlight border if selected column
    let border_style = if is_selected_column {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(column.name());

    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    // calculate card height (5 lines: top border, title, tags, padding, bottom border)
    let card_height = 5;
    let card_spacing = 1; // space between cards

    // determine scroll offset for this column
    let scroll_offset = if is_selected_column {
        app.scroll_offset
    } else {
        0
    };

    // render each task as a card, starting from scroll_offset
    let mut rendered = 0;
    for (i, task) in tasks.iter().enumerate().skip(scroll_offset) {
        let y_offset = rendered as u16 * (card_height + card_spacing);

        // stop if we run out of space
        if y_offset + card_height > inner_area.height {
            break;
        }

        let card_area = Rect {
            x: inner_area.x,
            y: inner_area.y + y_offset,
            width: inner_area.width,
            height: card_height,
        };

        draw_task_card(f, task, card_area, is_selected_column && i == app.selected_index);
        rendered += 1;
    }
}

/// draw a single task card
fn draw_task_card(f: &mut Frame, task: &crate::board::Task, area: Rect, is_selected: bool) {
    // card border style
    let border_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    // background color for selected task
    let bg_color = if is_selected {
        Color::DarkGray
    } else {
        Color::Reset
    };

    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(bg_color));

    let inner = card_block.inner(area);
    f.render_widget(card_block, area);

    // render task title and tags on separate lines
    if inner.height >= 2 {
        // truncate title to fit width
        let max_title_len = inner.width as usize;
        let truncated_title: String = task.title.chars().take(max_title_len).collect();

        // build tag string
        let tag_str = if !task.tags.is_empty() {
            format!("#{}", task.tags.join(" #"))
        } else {
            String::new()
        };

        let mut lines = vec![
            // Line 1: Title
            Line::from(Span::styled(
                truncated_title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
            ))
        ];

        // Line 2: Tags (if any)
        if !tag_str.is_empty() {
            lines.push(Line::from(Span::styled(
                tag_str,
                Style::default()
                    .fg(task.get_color())
                    .add_modifier(Modifier::DIM)
            )));
        }

        let content = Paragraph::new(lines);
        f.render_widget(content, inner);
    }
}

// draw footer with help text or input field
fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let text = match app.input_mode {
        InputMode::Normal => {
            vec![
                Line::from(vec![
                    Span::raw("hjkl/arrows: navigate | "),
                    Span::raw("Enter: open task | "),
                    Span::raw("a: add task | "),
                    Span::raw("t: add tag | "),
                    Span::raw("m: move | "),
                    Span::raw("d: delete | "),
                    Span::raw("?: help | "),
                    Span::raw("q: quit"),
                ])
            ]
        }
        InputMode::AddingTask => {
            vec![
                Line::from(vec![
                    Span::styled("Add Task: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.input_buffer),
                ]),
                Line::from("Press Enter to submit, Esc to cancel"),
            ]
        }
        InputMode::AddingTag => {
            vec![
                Line::from(vec![
                    Span::styled("Add Tag: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.input_buffer),
                ]),
                Line::from("Press Enter to submit, Esc to cancel"),
            ]
        }
        _ => vec![Line::from("")],
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

// draw task detail view
fn draw_task_detail(f: &mut Frame, app: &App) {
    let area = f.area();

    // get the selected task
    let column = app.board.get_column(app.selected_column);
    if app.selected_index >= column.len() {
        return;
    }
    let task = &column[app.selected_index];

    // check if we're editing the description
    let is_editing = app.input_mode == InputMode::EditingDescription;

    // create main container with context-aware title
    let title = if is_editing {
        " Task Details - EDITING DESCRIPTION (Enter for newline, Esc to save) "
    } else {
        " Task Details (Press Esc to close, e to edit description) "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);

    let inner = block.inner(area);
    f.render_widget(block, area);

    // split into sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Tags
            Constraint::Min(5),     // Description
        ])
        .split(inner);

    // title section
    let title_text = vec![
        Line::from(vec![
            Span::styled("Title: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&task.title),
        ]),
    ];
    let title_para = Paragraph::new(title_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title_para, sections[0]);

    // tags section
    let tags_str = if !task.tags.is_empty() {
        task.tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" ")
    } else {
        String::from("No tags")
    };
    let tags_text = vec![
        Line::from(vec![
            Span::styled("Tags: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(tags_str, Style::default().fg(task.get_color())),
        ]),
    ];
    let tags_para = Paragraph::new(tags_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(tags_para, sections[1]);

    // description section - show input field if editing, otherwise show text
    if is_editing {
        // Show editable input field
        let desc_para = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Description [EDITING]")
                .border_style(Style::default().fg(Color::Yellow)))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(desc_para, sections[2]);
    } else {
        // Show read-only description
        let desc_text = if task.description.is_empty() {
            "No description (press 'e' to add)"
        } else {
            &task.description
        };
        let desc_para = Paragraph::new(desc_text)
            .block(Block::default().borders(Borders::ALL).title("Description"))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(desc_para, sections[2]);
    }
}

// draw help view
fn draw_help(f: &mut Frame, _app: &App) {
    let area = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Help (Press Esc or ? to close) ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  h/← : Move left (previous column)"),
        Line::from("  j/↓ : Move down (next task)"),
        Line::from("  k/↑ : Move up (previous task)"),
        Line::from("  l/→ : Move right (next column)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Task Management:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  Enter : Open task details"),
        Line::from("  a     : Add new task to current column"),
        Line::from("  t     : Add tag to selected task"),
        Line::from("  m     : Move task to next column (TODO → IN PROGRESS → DONE)"),
        Line::from("  d     : Delete selected task"),
        Line::from("  e     : Edit description (when viewing task)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Predefined Tags:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("urgent", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("  : Red color (high priority)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("bug", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("     : Yellow color (needs fixing)"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("feature", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" : Green color (new feature)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Other:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  ?     : Show this help"),
        Line::from("  q     : Quit application"),
    ];

    let help_para = Paragraph::new(help_text);
    f.render_widget(help_para, inner);
}