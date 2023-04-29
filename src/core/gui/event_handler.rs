use std::{
    error::Error,
    ops::{Deref, DerefMut},
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};

pub struct EventHandler {
    pub receiver: Receiver<Action>,
    worker: EventWorker,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Exit,
    None,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let (sender, receiver) = unbounded();
        let worker = EventWorker::run("event_worker", sender).unwrap();
        Self { receiver, worker }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        EventHandler::new()
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.sender.send(Action::Exit).unwrap();
        self.thread.take().map(JoinHandle::join);
    }
}

pub struct EventWorker {
    name: String,
    thread: Option<JoinHandle<()>>,
    sender: Sender<Action>,
}

impl EventWorker {
    pub fn run(
        name: impl Into<String>,
        sender: Sender<Action>,
    ) -> Result<EventWorker, Box<dyn Error>> {
        let (tx, rx) = unbounded();
        let name = name.into();
        let handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || loop {
                if poll(Duration::from_millis(100)).unwrap() {
                    match read().unwrap() {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                        }) => sender.send(Action::Exit).unwrap(),
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('w'),
                            modifiers: KeyModifiers::NONE,
                        }) => sender.send(Action::Up).unwrap_or(()),
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('a'),
                            modifiers: KeyModifiers::NONE,
                        }) => sender.send(Action::Left).unwrap_or(()),
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('s'),
                            modifiers: KeyModifiers::NONE,
                        }) => sender.send(Action::Down).unwrap_or(()),
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('d'),
                            modifiers: KeyModifiers::NONE,
                        }) => sender.send(Action::Right).unwrap_or(()),
                        _ => (),
                    };
                }
                if let Ok(Action::Exit) = rx.try_recv() {
                    break;
                }
            })
            .unwrap();

        Ok(EventWorker {
            name,
            thread: Some(handle),
            sender: tx,
        })
    }
}

impl Deref for EventHandler {
    type Target = EventWorker;

    fn deref(&self) -> &Self::Target {
        &self.worker
    }
}

impl DerefMut for EventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.worker
    }
}
