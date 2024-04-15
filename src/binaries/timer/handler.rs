use crate::binaries::timer::App;
use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;

use super::app::CurrentScreen;
use super::app::CurrentlyEditing;

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == event::KeyEventKind::Release {
        return;
    }
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Char('+') => {
                app.current_screen = CurrentScreen::Editing;
                app.currently_editing = Some(CurrentlyEditing::Name);
            }
            KeyCode::Char('q') => {
                app.current_screen = CurrentScreen::Exiting;
            }
            KeyCode::Down => {
                app.timers.next();
            }
            KeyCode::Up => {
                app.timers.previous();
            }
            _ => {}
        },
        CurrentScreen::Exiting => match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('q') | KeyCode::Enter => {
                app.exit();
            }
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::Main;
            }
            _ => {}
        },
        CurrentScreen::Editing if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {
                            app.currently_editing = Some(CurrentlyEditing::Hour);
                        }
                        CurrentlyEditing::Hour => {
                            app.currently_editing = Some(CurrentlyEditing::Minute);
                        }
                        CurrentlyEditing::Minute => {
                            app.currently_editing = Some(CurrentlyEditing::Second);
                        }
                        CurrentlyEditing::Second => {
                            app.save_new_timer();
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {
                            app.name_input.pop();
                        }
                        CurrentlyEditing::Hour => {
                            app.hour_input.pop();
                        }
                        CurrentlyEditing::Minute => {
                            app.minute_input.pop();
                        }
                        CurrentlyEditing::Second => {
                            app.second_input.pop();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
                app.currently_editing = None;
            }
            KeyCode::Up => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {}
                        CurrentlyEditing::Hour => {
                            app.currently_editing = Some(CurrentlyEditing::Name);
                        }
                        CurrentlyEditing::Minute => {
                            app.currently_editing = Some(CurrentlyEditing::Hour);
                        }
                        CurrentlyEditing::Second => {
                            app.currently_editing = Some(CurrentlyEditing::Minute);
                        }
                    }
                }
            }
            KeyCode::Down => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {
                            app.currently_editing = Some(CurrentlyEditing::Hour);
                        }
                        CurrentlyEditing::Hour => {
                            app.currently_editing = Some(CurrentlyEditing::Minute);
                        }
                        CurrentlyEditing::Minute => {
                            app.currently_editing = Some(CurrentlyEditing::Second);
                        }
                        CurrentlyEditing::Second => {}
                    }
                }
            }
            KeyCode::Char(value) => {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Name => {
                            app.name_input.push(value);
                        }
                        CurrentlyEditing::Hour => {
                            app.hour_input.push(value);
                        }
                        CurrentlyEditing::Minute => {
                            app.minute_input.push(value);
                        }
                        CurrentlyEditing::Second => {
                            app.second_input.push(value);
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
