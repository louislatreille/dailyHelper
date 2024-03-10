use std::sync::mpsc;
use std::thread;

use crate::commands::Command;

enum ThreadMessage {
    Command(Box<dyn Command>),
    Shutdown,
}

pub struct ThreadHandler {
    tx: mpsc::Sender<ThreadMessage>,
}

impl ThreadHandler {
    pub fn start() -> ThreadHandler {
        let (tx, rx) = mpsc::channel::<ThreadMessage>();

        thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                match message {
                    ThreadMessage::Shutdown => {
                        println!("Shutdown command received, exiting thread.");
                        return;
                    }
                    ThreadMessage::Command(command) => {
                        command.execute();
                    }
                }
            }
        });

        ThreadHandler { tx }
    }

    pub fn send_command(&self, command: Box<dyn Command>) {
        if let Err(e) = self.tx.send(ThreadMessage::Command(command)) {
            println!("Error sending command: {}", e);
        }
    }

    pub fn shutdown(self) {
        if let Err(e) = self.tx.send(ThreadMessage::Shutdown) {
            println!("Error shutting down...: {}", e);
        }
    }
}
