pub mod app;
mod ui;

use super::Runnable;
use crate::binaries::json::app::App;

pub struct Json {
    app: App,
}

impl Runnable for Json {
    fn run(&mut self) -> Result<(), String> {
        match self.app.run_app() {
            Ok(_) => match self.app.print_json() {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Json {
    pub fn new() -> Self {
        Self { app: App::new() }
    }
}
