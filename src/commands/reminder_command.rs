use crate::commands::Command;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use std::thread::sleep;
use std::{thread, time::Duration};
use winrt_notification::{
    Action, Duration as NotificationDuration, InputType, Sound, Toast, ToastWithHandlers,
};

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
    fn schedule(&self, duration: Duration) {
        let message = self.message.clone();

        thread::spawn(move || {
            sleep(duration);

            let toast = Toast::new(Toast::POWERSHELL_APP_ID)
                .title(&message)
                .text1("Need to postpone?")
                .sound(Some(Sound::SMS))
                .duration(NotificationDuration::Long)
                .action(Action {
                    content: "1".to_string(),
                    arguments: "1".to_string(),
                    place_to_context_menu: false,
                })
                .action(Action {
                    content: "3".to_string(),
                    arguments: "3".to_string(),
                    place_to_context_menu: false,
                })
                .action(Action {
                    content: "5".to_string(),
                    arguments: "5".to_string(),
                    place_to_context_menu: false,
                })
                .input(
                    InputType::text_with_placeholder("minutes"),
                    "custom_minutes",
                )
                .action(Action {
                    content: "Custom".to_string(),
                    arguments: "custom_minutes".to_string(),
                    place_to_context_menu: false,
                });

            ToastWithHandlers::new(toast)
                .on_activate(move |args| {
                    let str_args = match args.get_arguments() {
                        Some(args) => match args {
                            Ok(args) => args,
                            Err(e) => {
                                println!(
                                    "Error receiving arguments from notification input: {}",
                                    e
                                );
                                return Ok(());
                            }
                        },
                        None => {
                            print!("Received no argument from notification input");
                            return Ok(());
                        }
                    };

                    println!("Got args {}", str_args);

                    match i64::from_str_radix(&str_args, 10) {
                        Ok(minutes) => {
                            let now = Utc::now();
                            let future = now
                                .checked_add_signed(ChronoDuration::minutes(minutes))
                                .unwrap();

                            ReminderCommand::new(future, message.clone()).execute();
                        }
                        Err(_) => match args.get_user_input() {
                            Some(args) => match args {
                                Ok(args) => match args.get(&str_args) {
                                    Some(minutes) => match i64::from_str_radix(&minutes, 10) {
                                        Ok(minutes) => {
                                            let now = Utc::now();
                                            let future = now
                                                .checked_add_signed(ChronoDuration::minutes(
                                                    minutes,
                                                ))
                                                .unwrap();

                                            ReminderCommand::new(future, message.clone()).execute();
                                        }
                                        Err(e) => println!("Invalid number received. {}", e),
                                    },
                                    None => println!("No minute argument received",),
                                },
                                Err(e) => println!(
                                    "Error receiving arguments from notification input: {}",
                                    e
                                ),
                            },
                            None => print!("Received no argument from notification input"),
                        },
                    }

                    Ok(())
                })
                .show()
                .expect("Unable to send notification");
        });
    }
}

impl Command for ReminderCommand {
    fn execute(&self) {
        let now = Utc::now();
        let future_in_utc = self.future_time.with_timezone(&Utc);
        match (future_in_utc - now).to_std() {
            Ok(duration_until) => self.schedule(duration_until),
            Err(e) => {
                println!(
                    "Reminder received with an invalid future time: {}. {}",
                    self.future_time, e
                );
            }
        }
    }
}
