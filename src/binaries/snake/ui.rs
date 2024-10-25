use collision_detection::check_collisions;
use ratatui::{symbols::Marker, widgets::canvas::Canvas, Frame};

use super::{app::App, game::GameState, letters::Word};

pub fn render_frame(app: &mut App, frame: &mut Frame, width: f64, height: f64) {
    let area = frame.size();

    if check_collisions(&app.snake.head, &app.snake.body) {
        app.game.game_over();
    }
    if check_collisions(&app.snake.head, &app.walls) {
        app.game.game_over();
    }

    if check_collisions(&app.snake.head, &app.point) {
        app.snake.grow();
        app.game.increase_score();
        app.point.create_new_point();
    }

    if app.game.state == GameState::Running && app.game.frame_num == 0 {
        app.snake.move_snake();
    }

    frame.render_widget(
        Canvas::default()
            .x_bounds([-width / 2.0, width / 2.0])
            .y_bounds([-height, height])
            .marker(Marker::HalfBlock)
            .paint(|ctx| {
                ctx.draw(&app.walls);

                ctx.layer();

                if app.game.state != GameState::Startup {
                    ctx.print(
                        -width / 2.0 + 3.0,
                        height - 4.0,
                        format!("Score: {}", app.game.score),
                    );
                }

                ctx.layer();

                match app.game.state {
                    GameState::Running | GameState::Paused => {
                        ctx.draw(&app.snake);
                        ctx.draw(&app.point);
                    }
                    GameState::GameOver => {
                        ctx.draw(&Word::new("gameover".to_string(), -27.0));
                        ctx.print(-9.0, -5.0, "Press R to restart");
                    }
                    GameState::Startup => {
                        ctx.draw(&Word::new("ratatui snake".to_string(), -48.0));
                        ctx.print(-15.0, -5.0, "Press any character to start");
                    }
                }
            }),
        area,
    )
}
