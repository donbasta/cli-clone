use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{self, Stderr};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::enable_raw_mode,
};
use ratatui::{prelude::*, widgets::*};

use super::ui::render_frame;

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
    pub name: String,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Timer {
    pub fn to_list_item(&self) -> ListItem {
        ListItem::new(Line::from(Span::styled(
            format!("{: <25}", self.name),
            Style::default().fg(Color::Yellow),
        )))
    }
}

#[derive(Debug)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Timer>,
    pub last_selected: Option<usize>,
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
    pub fn get_selected_index(&self) -> Option<usize> {
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
            terminal.draw(|f| render_frame(self, f))?;

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
}
