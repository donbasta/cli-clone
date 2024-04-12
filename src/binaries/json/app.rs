use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::collections::HashMap;
use std::io::{self, Stderr};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::enable_raw_mode,
};
use ratatui::prelude::*;

use super::ui::render_frame;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Debug)]
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Ok(do_print) = res {
            if do_print {
                self.print_json()?;
            }
        } else if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stderr>>) -> io::Result<bool> {
        loop {
            terminal.draw(|f| render_frame(self, f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match self.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => {
                            self.current_screen = CurrentScreen::Editing;
                            self.currently_editing = Some(CurrentlyEditing::Key);
                        }
                        KeyCode::Char('q') => {
                            self.current_screen = CurrentScreen::Exiting;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') | KeyCode::Char('q') => {
                            return Ok(false);
                        }
                        _ => {}
                    },
                    CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        self.save_key_value();
                                        self.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.pop();
                                    }
                                    CurrentlyEditing::Value => {
                                        self.value_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Main;
                            self.currently_editing = None;
                        }
                        KeyCode::Tab => {
                            self.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        self.value_input.push(value);
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
