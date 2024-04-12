use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{self, Stderr};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::enable_raw_mode,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::ui::utils::centered_rect;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Name,
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
pub struct Timer {
    name: String,
    hour: u32,
    minute: u32,
    second: u32,
}

impl Timer {
    fn to_list_item(&self) -> ListItem {
        ListItem::new(Line::from(Span::styled(
            format!("{: <25}", self.name),
            Style::default().fg(Color::Yellow),
        )))
    }
}

#[derive(Debug)]
pub struct StatefulList {
    state: ListState,
    items: Vec<Timer>,
    last_selected: Option<usize>,
}

impl StatefulList {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            last_selected: None,
        }
    }
    fn add(&mut self, new_timer: Timer) {
        self.items.push(new_timer);
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    fn set_last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
    fn get_selected_index(&self) -> Option<usize> {
        return self.state.selected();
    }
}

#[derive(Debug)]
pub struct App {
    pub name_input: String,
    pub hour_input: String,
    pub minute_input: String,
    pub second_input: String,

    pub timers: StatefulList,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            name_input: String::new(),
            hour_input: String::new(),
            minute_input: String::new(),
            second_input: String::new(),

            timers: StatefulList::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
    pub fn save_new_timer(&mut self) {
        self.timers.add(Timer {
            name: self.name_input.clone(),
            hour: self.hour_input.clone().parse::<u32>().unwrap(),
            minute: self.minute_input.clone().parse::<u32>().unwrap(),
            second: self.second_input.clone().parse::<u32>().unwrap(),
        });

        self.name_input = String::new();
        self.hour_input = String::new();
        self.minute_input = String::new();
        self.second_input = String::new();
        self.currently_editing = None;
        self.timers.set_last();
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        let _ = self.run(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.render_frame(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match self.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('+') => {
                            self.current_screen = CurrentScreen::Editing;
                            self.currently_editing = Some(CurrentlyEditing::Name);
                        }
                        KeyCode::Char('q') => {
                            self.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Down => {
                            self.timers.next();
                        }
                        KeyCode::Up => {
                            self.timers.previous();
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('q') | KeyCode::Enter => {
                            return Ok(());
                        }
                        KeyCode::Char('n') => {
                            self.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                    CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Name => {
                                        self.currently_editing = Some(CurrentlyEditing::Hour);
                                    }
                                    CurrentlyEditing::Hour => {
                                        self.currently_editing = Some(CurrentlyEditing::Minute);
                                    }
                                    CurrentlyEditing::Minute => {
                                        self.currently_editing = Some(CurrentlyEditing::Second);
                                    }
                                    CurrentlyEditing::Second => {
                                        self.save_new_timer();
                                        self.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Name => {
                                        self.name_input.pop();
                                    }
                                    CurrentlyEditing::Hour => {
                                        self.hour_input.pop();
                                    }
                                    CurrentlyEditing::Minute => {
                                        self.minute_input.pop();
                                    }
                                    CurrentlyEditing::Second => {
                                        self.second_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Main;
                            self.currently_editing = None;
                        }
                        KeyCode::Up => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Name => {}
                                    CurrentlyEditing::Hour => {
                                        self.currently_editing = Some(CurrentlyEditing::Name);
                                    }
                                    CurrentlyEditing::Minute => {
                                        self.currently_editing = Some(CurrentlyEditing::Hour);
                                    }
                                    CurrentlyEditing::Second => {
                                        self.currently_editing = Some(CurrentlyEditing::Minute);
                                    }
                                }
                            }
                        }
                        KeyCode::Down => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Name => {
                                        self.currently_editing = Some(CurrentlyEditing::Hour);
                                    }
                                    CurrentlyEditing::Hour => {
                                        self.currently_editing = Some(CurrentlyEditing::Minute);
                                    }
                                    CurrentlyEditing::Minute => {
                                        self.currently_editing = Some(CurrentlyEditing::Second);
                                    }
                                    CurrentlyEditing::Second => {}
                                }
                            }
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Name => {
                                        self.name_input.push(value);
                                    }
                                    CurrentlyEditing::Hour => {
                                        self.hour_input.push(value);
                                    }
                                    CurrentlyEditing::Minute => {
                                        self.minute_input.push(value);
                                    }
                                    CurrentlyEditing::Second => {
                                        self.second_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "🍅 Pomodoro Timer 🍅",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        frame.render_widget(title, chunks[0]);

        let list_items: Vec<ListItem> = self
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

        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(chunks[1]);

        frame.render_stateful_widget(timer_list, body_chunks[0], &mut self.timers.state);

        if let Some(idx) = self.timers.get_selected_index() {
            let detailed_timer = Paragraph::new(Text::styled(
                format!(
                    "{}:{}:{}",
                    self.timers.items[idx].hour,
                    self.timers.items[idx].minute,
                    self.timers.items[idx].second
                ),
                Style::default().fg(Color::Green),
            ))
            .block(Block::default().borders(Borders::ALL));
            frame.render_widget(detailed_timer, body_chunks[1]);
        }

        let current_navigation_text = vec![
            match self.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal mode", Style::default().fg(Color::Green))
                }
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
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {
                            Span::styled("Editing Timer Name", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Hour => Span::styled(
                            "Editing Timer Hour",
                            Style::default().fg(Color::LightGreen),
                        ),
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

        let current_keys_hint = {
            match self.current_screen {
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

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        frame.render_widget(mode_footer, footer_chunks[0]);
        frame.render_widget(key_notes_footer, footer_chunks[1]);

        if let Some(editing) = &self.currently_editing {
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

            let name_input_text = Paragraph::new(self.name_input.clone()).block(name_block);
            let hour_input_text = Paragraph::new(self.hour_input.clone()).block(hour_block);
            let minute_input_text = Paragraph::new(self.minute_input.clone()).block(minute_block);
            let second_input_text = Paragraph::new(self.second_input.clone()).block(second_block);

            frame.render_widget(name_input_text, popup_chunks[0]);
            frame.render_widget(hour_input_text, popup_chunks[1]);
            frame.render_widget(minute_input_text, popup_chunks[2]);
            frame.render_widget(second_input_text, popup_chunks[3]);
        }

        if let CurrentScreen::Exiting = self.current_screen {
            frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
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
}
