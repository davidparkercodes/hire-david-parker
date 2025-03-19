use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event receiver channel.
    pub receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    _handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(Duration::from_secs(0));

                    if event::poll(timeout).expect("failed to poll events") {
                        match event::read().expect("failed to read event") {
                            CrosstermEvent::Key(e) => {
                                if sender.send(Event::Key(e)).is_err() {
                                    return;
                                }
                            }
                            CrosstermEvent::Resize(w, h) => {
                                if sender.send(Event::Resize(w, h)).is_err() {
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(Event::Tick).is_err() {
                            return;
                        }
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            receiver,
            _handler: handler,
        }
    }
}