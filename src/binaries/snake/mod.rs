// The snake game code is taken from: https://github.com/kriskw1999/ratatui-snake, including:
// the logics and the objects (game, letters, point, snake, walls)

use app::App;

use super::{AppResult, Runnable};

pub mod app;
mod game;
mod letters;
mod point;
mod snake;
mod ui;
mod walls;

pub struct Snake {
    app: App,
}

impl Runnable for Snake {
    fn run(&mut self) -> AppResult<()> {
        match self.app.run_app() {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}

impl Snake {
    pub fn new() -> Self {
        Self { app: App::new() }
    }
}
