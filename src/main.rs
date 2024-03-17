mod commands;
mod thread_handler; // Declaring the module

extern crate chrono;
extern crate chrono_tz;
extern crate winrt_notification;
use chrono::{Datelike, Local, NaiveTime, TimeZone, Timelike, Utc};
use thread_handler::ThreadHandler;
// use windows::{core::w, Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID};

use crate::commands::{PrintCommand, ReminderCommand, SleepCommand}; // Using the struct and enum from the module

fn main() {
    // This doesn't seem to be working properly
    // unsafe {
    //     if let Err(e) =
    //         SetCurrentProcessExplicitAppUserModelID(w!("com.louislatreille.dailyhelper"))
    //     {
    //         println!("Error setting app user ID {}.", e)
    //     }
    // }

    let handler = ThreadHandler::start();

    loop {
        let mut input = String::new();
        println!("Enter a command:");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("Error reading user input {}.", e)
        }

        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match parts.as_slice() {
            // ["reminder", datetime, message] => match  datetime.parse::<DateTime<Utc>>() {
            ["reminder", datetime, message] => {
                match NaiveTime::parse_from_str(datetime, "%H:%M:%S") {
                    Ok(duration) => {
                        let local = Local::now();
                        match Local.with_ymd_and_hms(
                            local.year(),
                            local.month(),
                            local.day(),
                            duration.hour(),
                            duration.minute(),
                            duration.second(),
                        ) {
                            chrono::LocalResult::Single(local_datetime) => {
                                println!(
                                    "Will remind you \"{}\" at {}",
                                    message,
                                    local_datetime.format("%H:%M:%S")
                                );
                                
                                handler.send_command(Box::new(ReminderCommand::new(
                                    local_datetime.with_timezone(&Utc),
                                    message.to_string(),
                                )))
                            }
                            _ => println!("Unexpected error. Date doesn't have year-month-day hour:minute:seconds."),
                        }
                    }
                    Err(e) => println!("Error parsing date {}. {}.", datetime, e),
                }
            }
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
