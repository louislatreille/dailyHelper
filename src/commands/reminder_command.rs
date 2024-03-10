use crate::commands::Command;

use chrono::{DateTime, Utc};
use std::thread;
use std::thread::sleep;
use winrt_notification::{Duration, Sound, Toast};

pub struct ReminderCommand {
    future_time: DateTime<Utc>,
    message: String,
}

impl ReminderCommand {
    pub fn new(future_time: DateTime<Utc>, message: String) -> Self {
        ReminderCommand {
            future_time,
            message,
        }
    }
}

impl Command for ReminderCommand {
    fn execute(&self) {
        let now = Utc::now();
        match (self.future_time - now).to_std() {
            Ok(duration_until) => {
                println!(
                    "Reminder command received. Current time: {:?}, Scheduled task for: {:?}",
                    now, self.future_time
                );

                let message = self.message.clone();

                thread::spawn(move || {
                    sleep(duration_until);

                    Toast::new("com.louislatreille.dailyhelper")
                        .title(&message)
                        .text1(&message)
                        .sound(Some(Sound::SMS))
                        .duration(Duration::Short)
                        .show()
                        .unwrap();
                });
            }
            Err(e) => {
                println!("Reminder received with an invalid future time. {}", e);
            }
        }
    }
}
