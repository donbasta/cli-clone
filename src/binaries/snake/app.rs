use std::io::{self, stdout, Error};

use super::{
    game::{Direction, Game, GameState},
    point::Point,
    snake::{self, Snake},
    ui::render_frame,
    walls::Walls,
};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub struct App {
    pub running: bool,
    pub snake: Snake,
    pub game: Game,
    pub point: Point,
    pub walls: Walls,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            snake: Snake::new(),
            game: Game::new(),
            point: Point::new(0 as f64, 0 as f64),
            walls: Walls::new(0 as f64, 0 as f64),
        }
    }
    pub fn run_app(&mut self) -> io::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        if let Err(_) = terminal.clear() {
            return Err(Error::other("error while initializing terminal."));
        }

        let size = terminal.size()?;
        let height = size.height as f64;
        let width = size.width as f64;
        self.point = Point::new(width, height);
        self.walls = Walls::new(width, height);

        loop {
            self.game.increase_frame_num();

            terminal.draw(|frame| render_frame(self, frame, width, height))?;

            if event::poll(std::time::Duration::from_millis(8))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if self.game.state == GameState::Startup {
                            self.game.state = GameState::Running;
                        } else {
                            match key.code {
                                KeyCode::Char('q') => {
                                    break;
                                }
                                KeyCode::Char('p') => {
                                    if self.game.state == GameState::Paused {
                                        self.game.state = GameState::Running;
                                    } else if GameState::Running == self.game.state {
                                        self.game.state = GameState::Paused;
                                    }
                                }
                                KeyCode::Char('a') | KeyCode::Left => {
                                    if self.snake.head.direction != Direction::Right {
                                        self.snake.change_direction(Direction::Left);
                                    }
                                }
                                KeyCode::Char('d') | KeyCode::Right => {
                                    if self.snake.head.direction != Direction::Left {
                                        self.snake.change_direction(Direction::Right);
                                    }
                                }
                                KeyCode::Char('w') | KeyCode::Up => {
                                    if self.snake.head.direction != Direction::Down {
                                        self.snake.change_direction(Direction::Up);
                                    }
                                }
                                KeyCode::Char('s') | KeyCode::Down => {
                                    if self.snake.head.direction != Direction::Up {
                                        self.snake.change_direction(Direction::Down);
                                    }
                                }
                                KeyCode::Char('r') | KeyCode::Char('R') => {
                                    if self.game.state == GameState::GameOver {
                                        self.game.restart();
                                        self.snake = snake::Snake::new();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
