use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::collections::HashMap;
use std::io::{self, Stderr};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::enable_raw_mode,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

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
            terminal.draw(|f| self.render_frame(f))?;

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
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
    fn render_frame(&self, frame: &mut Frame) {
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
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        frame.render_widget(title, chunks[0]);
        let mut list_items = Vec::<ListItem>::new();

        for key in self.pairs.keys() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25} : {}", key, self.pairs.get(key).unwrap()),
                Style::default().fg(Color::Yellow),
            ))));
        }
        let list = List::new(list_items);
        frame.render_widget(list, chunks[1]);

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
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
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
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Editing => Span::styled(
                    "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(q) to quit / (e) to make new pair",
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
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = self.centered_rect(60, 25, frame.size());
            frame.render_widget(popup_block, area);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            match editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_style),
            };

            let key_text = Paragraph::new(self.key_input.clone()).block(key_block);
            frame.render_widget(key_text, popup_chunks[0]);

            let value_text = Paragraph::new(self.value_input.clone()).block(value_block);
            frame.render_widget(value_text, popup_chunks[1]);
        }

        if let CurrentScreen::Exiting = self.current_screen {
            frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the buffer as json? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = self.centered_rect(60, 25, frame.size());
            frame.render_widget(exit_paragraph, area);
        }
    }
}
