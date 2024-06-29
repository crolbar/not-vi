use std::{sync::mpsc, thread};
use crossterm::event::{self, Event};
use anyhow::Result;

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                if let Some(e) = Some(event::read().unwrap()) {
                    tx.send(e).unwrap();
                }
            }
        });

        Self { rx }
    }

    pub fn recv(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
