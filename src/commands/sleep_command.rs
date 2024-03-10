use crate::commands::Command;
use std::thread;
use std::time::Duration;

pub struct SleepCommand {
    duration: u32,
}

impl SleepCommand {
    pub fn new(duration: u32) -> Self {
        SleepCommand { duration }
    }
}

impl Command for SleepCommand {
    fn execute(&self) {
        println!(
            "Sleep command received, sleeping for {} seconds",
            self.duration
        );
        thread::sleep(Duration::from_secs(self.duration as u64));
    }
}
