use crate::binaries::counter::App;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Left => app.decrement_counter(),
        KeyCode::Right => app.increment_counter(),
        _ => {}
    }
}
