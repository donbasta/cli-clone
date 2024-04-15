use std::{
    sync::mpsc::{self},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

use crate::binaries::AppResult;

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<EventType>,
    receiver: mpsc::Receiver<EventType>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout).expect("no events available") {
                        if let Ok(()) = match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(EventType::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(EventType::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(EventType::Resize(w, h)),
                            _ => unimplemented!(),
                        } {}
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> AppResult<EventType> {
        Ok(self.receiver.recv()?)
    }
}
