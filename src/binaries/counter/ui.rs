use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use super::app::App;

pub fn render_frame(app: &mut App, frame: &mut Frame) {
    let title = Title::from(" Counter App Tutorial ".bold());
    let instructions = Title::from(Line::from(vec![
        " Decrement ".into(),
        "<Left>".blue().bold(),
        " Increment ".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        app.counter.to_string().yellow(),
    ])]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);
    frame.render_widget(paragraph, frame.size());
}
