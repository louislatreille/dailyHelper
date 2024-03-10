mod commands;
mod thread_handler; // Declaring the module

extern crate chrono;
extern crate winrt_notification;
use chrono::{DateTime, Utc};
use thread_handler::ThreadHandler;
use windows::{core::w, Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID};

use crate::commands::{PrintCommand, ReminderCommand, SleepCommand}; // Using the struct and enum from the module

fn main() {
    unsafe {
        if let Err(e) =
            SetCurrentProcessExplicitAppUserModelID(w!("com.louislatreille.dailyhelper"))
        {
            println!("Error setting app user ID {}.", e)
        }
    }

    let handler = ThreadHandler::start();

    loop {
        let mut input = String::new();
        // println!("Enter a message to display in a toast notification:");
        println!("Enter a command:");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("Error reading user input {}.", e)
        }

        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match parts.as_slice() {
            ["reminder", datetime, message] => match datetime.parse::<DateTime<Utc>>() {
                Ok(duration) => handler.send_command(Box::new(ReminderCommand::new(
                    duration,
                    message.to_string(),
                ))),
                Err(e) => println!("Error parsing date {}.", e),
            },
            ["sleep", secs] => match secs.parse::<u32>() {
                Ok(duration) => handler.send_command(Box::new(SleepCommand::new(duration))),
                Err(_) => println!("Error: Failed to parse sleep duration."),
            },
            ["print", message] => {
                handler.send_command(Box::new(PrintCommand::new(message.to_string())))
            }
            ["shutdown"] => {
                handler.shutdown();
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}
