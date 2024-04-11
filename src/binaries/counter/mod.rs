pub mod app;

use super::Runnable;
use crate::binaries::counter::app::App;

pub struct Counter {
    app: App,
}

impl Runnable for Counter {
    fn run(&mut self) -> Result<(), String> {
        match self.app.run_app() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            app: App::default(),
        }
    }
}
