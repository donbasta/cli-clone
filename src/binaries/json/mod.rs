pub mod app;
mod ui;

use super::{AppResult, Runnable};
use crate::binaries::json::app::App;

pub struct Json {
    app: App,
}

impl Runnable for Json {
    fn run(&mut self) -> AppResult<()> {
        match self.app.run_app() {
            Ok(_) => match self.app.print_json() {
                Ok(_) => Ok(()),
                Err(err) => Err(Box::new(err)),
            },
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl Json {
    pub fn new() -> Self {
        Self { app: App::new() }
    }
}
