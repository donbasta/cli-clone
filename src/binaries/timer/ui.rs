use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::ui::utils::centered_rect;

use super::app::{App, CurrentScreen, CurrentlyEditing};

pub fn render_title(area: Rect, frame: &mut Frame) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "🍅 Pomodoro Timer 🍅",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, area);
}

pub fn render_timers_list(app: &mut App, area: Rect, frame: &mut Frame) {
    let list_items: Vec<ListItem> = app
        .timers
        .items
        .iter()
        .enumerate()
        .map(|(_, timer_item)| timer_item.to_list_item())
        .collect();

    let timer_list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().red().italic())
        .highlight_symbol("> ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(timer_list, area, &mut app.timers.state);
}

pub fn render_detailed_timer(app: &mut App, area: Rect, frame: &mut Frame) {
    if let Some(idx) = app.timers.get_selected_index() {
        let detailed_timer = Paragraph::new(Text::styled(
            format!(
                "{}:{}:{}",
                app.timers.items[idx].hour,
                app.timers.items[idx].minute,
                app.timers.items[idx].second
            ),
            Style::default().fg(Color::Green),
        ))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(detailed_timer, area);
    }
}

pub fn render_left_footer(app: &mut App, frame: &mut Frame, area: Rect) {
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting mode", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Name => {
                        Span::styled("Editing Timer Name", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Hour => {
                        Span::styled("Editing Timer Hour", Style::default().fg(Color::LightGreen))
                    }
                    CurrentlyEditing::Minute => {
                        Span::styled("Editing Timer Minute", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Second => Span::styled(
                        "Editing Timer Second",
                        Style::default().fg(Color::LightGreen),
                    ),
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(mode_footer, area);
}

pub fn render_right_footer(app: &App, frame: &mut Frame, area: Rect) {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (+) to add new",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(up/down) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (+) to add new",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_notes_footer, area);
}

pub fn render_editing_popup(app: &mut App, frame: &mut Frame) {
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new timer")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(50, 50, frame.size());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        let mut name_block = Block::default().title("Name").borders(Borders::ALL);
        let mut hour_block = Block::default().title("Hour").borders(Borders::ALL);
        let mut minute_block = Block::default().title("Minute").borders(Borders::ALL);
        let mut second_block = Block::default().title("Second").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Name => name_block = name_block.style(active_style),
            CurrentlyEditing::Hour => hour_block = hour_block.style(active_style),
            CurrentlyEditing::Minute => minute_block = minute_block.style(active_style),
            CurrentlyEditing::Second => second_block = second_block.style(active_style),
        };

        let name_input_text = Paragraph::new(app.name_input.clone()).block(name_block);
        let hour_input_text = Paragraph::new(app.hour_input.clone()).block(hour_block);
        let minute_input_text = Paragraph::new(app.minute_input.clone()).block(minute_block);
        let second_input_text = Paragraph::new(app.second_input.clone()).block(second_block);

        frame.render_widget(name_input_text, popup_chunks[0]);
        frame.render_widget(hour_input_text, popup_chunks[1]);
        frame.render_widget(minute_input_text, popup_chunks[2]);
        frame.render_widget(second_input_text, popup_chunks[3]);
    }
}

pub fn render_exiting_dialogue(app: &mut App, frame: &mut Frame) {
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.size());
        let popup_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to exit the timer app? (y/n)",
            Style::default().fg(Color::Red),
        );
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

pub fn render_frame(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    render_title(chunks[0], frame);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    render_timers_list(app, body_chunks[0], frame);
    render_detailed_timer(app, body_chunks[1], frame);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    render_left_footer(app, frame, footer_chunks[0]);
    render_right_footer(app, frame, footer_chunks[1]);

    render_editing_popup(app, frame);
    render_exiting_dialogue(app, frame);
}
