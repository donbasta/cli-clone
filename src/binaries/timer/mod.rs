pub mod app;
mod handler;
mod ui;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use self::{handler::handle_key_event, ui::render_frame};

use super::{AppResult, Runnable};
use crate::{
    binaries::timer::app::App,
    event::{EventHandler, EventType},
    ui::tui::Tui,
};

pub struct Timer {
    app: App,
}

impl Runnable for Timer {
    fn run(&mut self) -> AppResult<()> {
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;

        while self.app.running {
            tui.terminal
                .draw(|frame| render_frame(&mut self.app, frame))?;

            match tui.events.next()? {
                EventType::Tick => self.app.tick(),
                EventType::Key(key_event) => handle_key_event(&mut self.app, key_event),
                EventType::Mouse(_) => {}
                EventType::Resize(_, _) => {}
            }
        }

        tui.restore()?;
        Ok(())
    }
}

impl Timer {
    pub fn new() -> Self {
        Self { app: App::new() }
    }
}
