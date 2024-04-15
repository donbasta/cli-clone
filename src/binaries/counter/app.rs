use crate::binaries::AppResult;
use crate::event::EventHandler;
use crate::event::EventType;
use crate::ui::tui::Tui;
use std::io;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::handler::handle_key_event;
use super::ui::render_frame;

#[derive(Debug)]
pub struct App {
    pub counter: i8,
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            running: true,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_app(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;

        while self.running {
            tui.terminal.draw(|frame| render_frame(self, frame))?;

            match tui.events.next()? {
                EventType::Tick => self.tick(),
                EventType::Key(key_event) => handle_key_event(self, key_event),
                EventType::Mouse(_) => {}
                EventType::Resize(_, _) => {}
            }
        }

        tui.restore()?;
        Ok(())
    }

    pub fn tick(&self) {}

    pub fn exit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
