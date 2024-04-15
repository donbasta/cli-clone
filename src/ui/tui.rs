use std::io::stderr;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};
use ratatui::prelude::*;

use crate::{binaries::AppResult, event::EventHandler};

pub struct Tui<B: Backend> {
    pub terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self) -> AppResult<()> {
        enable_raw_mode()?;
        execute!(stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn restore(&mut self) -> AppResult<()> {
        disable_raw_mode()?;
        execute!(stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        // self.events.cleanup();
        Ok(())
    }
}
