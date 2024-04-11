pub mod app;

use super::Runnable;
use crate::binaries::timer::app::App;

pub struct Timer {
    app: App,
}

impl Runnable for Timer {
    fn run(&mut self) -> Result<(), String> {
        match self.app.run_app() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            app: App::default(),
        }
    }
}
