use chrono;
use std::sync::mpsc::{self, TryRecvError};
use std::{thread, time};
use winrt_notification::{Duration, Sound, Toast};

pub struct Reminder {
    pub title: String,
    pub text: String,
    pub time: i64,
}

pub struct CommandsManager {
    commands_sender: mpsc::Sender<String>,
}

impl CommandsManager {
    pub fn new() -> CommandsManager {
        let (commands_sender, commands_receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) =
            mpsc::channel();

        thread::spawn(move || {
            let mut reminders = Vec::new();

            loop {
                let text_command = match commands_receiver.try_recv() {
                    Ok(text_command) => text_command,
                    Err(TryRecvError::Empty) => "No command".to_string(),
                    Err(TryRecvError::Disconnected) => {
                        panic!("Error while listening for incoming commands. It is likely there are no more command dispatchers.")
                    }
                };

                if text_command != "No command" {
                    let reminder_time = chrono::offset::Utc::now().timestamp() + 5;
                    println!("Queuing reminder for: {}", reminder_time);

                    reminders.push(Reminder {
                        title: text_command.clone(),
                        text: text_command.clone(),
                        time: reminder_time,
                    });
                }

                let reminders_new =
                    reminders
                        .into_iter()
                        .fold((Vec::new(), Vec::new()), |mut acc, reminder| {
                            let now = chrono::offset::Utc::now().timestamp();

                            if reminder.time <= now {
                                acc.0.push(reminder);
                            } else {
                                acc.1.push(reminder);
                            }

                            acc
                        });

                println!("Reminders due: {}", reminders_new.0.len());
                println!("Reminders not due: {}", reminders_new.1.len());

                reminders_new.0.iter().for_each(|reminder: &Reminder| {
                    // Display toast notification
                    Toast::new(Toast::POWERSHELL_APP_ID)
                        .title(&reminder.title)
                        .text1(&reminder.text)
                        .sound(Some(Sound::SMS))
                        .duration(Duration::Short)
                        .show()
                        .unwrap();
                });

                reminders = reminders_new.1;

                thread::sleep(time::Duration::from_millis(1000));
            }
        });

        CommandsManager { commands_sender }
    }

    pub fn queue_reminder(&mut self, text_command: &str) {
        self.commands_sender.send(text_command.to_string()).unwrap();
    }
}
