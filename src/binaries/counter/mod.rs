pub mod app;
mod handler;
mod ui;

use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};

use self::{handler::handle_key_event, ui::render_frame};

use super::{AppResult, Runnable};
use crate::{
    binaries::counter::app::App,
    event::{EventHandler, EventType},
    ui::tui::Tui,
};

pub struct Counter {
    app_state: App,
}

impl Runnable for Counter {
    fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;

        while self.app_state.running {
            tui.terminal
                .draw(|frame| render_frame(&mut self.app_state, frame))?;

            match tui.events.next()? {
                EventType::Tick => self.app_state.tick(),
                EventType::Key(key_event) => handle_key_event(&mut self.app_state, key_event),
                EventType::Mouse(_) => {}
                EventType::Resize(_, _) => {}
            }
        }

        tui.restore()?;
        Ok(())
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            app_state: App::new(),
        }
    }
}
