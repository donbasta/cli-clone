pub mod app;
mod ui;

use super::{AppResult, Runnable};
use crate::binaries::timer::app::App;

pub struct Timer {
    app: App,
}

impl Runnable for Timer {
    fn run(&mut self) -> AppResult<()> {
        match self.app.run_app() {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self { app: App::new() }
    }
}
