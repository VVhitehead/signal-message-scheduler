use chrono::{NaiveDate, Duration, Local, NaiveTime, NaiveDateTime, TimeZone};
use chrono_tz::Europe::Berlin; // <- MODIFY to match your local time zone!
use inquire::{Select, InquireError, Confirm, Text, validator::Validation, CustomType, DateSelect, required};
use std::process::Command;
use std::{thread, println};
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH, Duration as SystemDuration};
use std::io::{self, Write};
use colored::Colorize;

use crate::contact::{Contact, MessageExpiration};


pub(crate) fn run_dialogue(list_of_contacts: String) {
    let account_number = get_account_number(&list_of_contacts);
    let contacts = get_recipients(&list_of_contacts);
    let filtered_contacts: Vec<_> = contacts.iter().filter(|&contact| !(contact.name.is_empty() && contact.profile_name.is_empty())).cloned().collect();
    let choice = Select::new("Write and schedule a message for:", filtered_contacts).prompt();

    match choice {
        Ok(choice) => {
        if choice.id == 0 {
            let confirmed = Confirm::new(&(format!("You want to send a self-note to: {}({})?", choice.profile_name.bold().blue(), choice.number.italic().bright_purple())))
                    .with_default(true)
                    .prompt()
                    .unwrap();
            if confirmed {
                    let message_time = NaiveDateTime::parse_from_str(&format!("{} {}", pick_date().unwrap(), pick_time().unwrap()), "%Y-%m-%d %H:%M:%S").expect("Full DateTime string");
                    let message = store_message().unwrap();
                    countdown(message_time);
                    if send_message_to_self(message.clone()) {
                        println!("\"{}\" sent to {}(as note to {}) @ {}", message.blink().bold().blue(), choice.number.bright_green().italic(), "SELF".bright_green().italic(), 
                                 message_time.to_string().italic().underline().bright_purple());
                    }
            } else {
                println!("{}", "Canceled!".bold().red());
            }
            } else {
                let confirmed = Confirm::new(&(format!("You want to message: {} - {}({})?", choice.name.bold().green(), choice.profile_name.italic().bright_purple(), choice.number.underline().red())))
                        .with_default(true)
                        .prompt()
                        .unwrap();
                if confirmed {
                    let message_time = NaiveDateTime::parse_from_str(&format!("{} {}", pick_date().unwrap(), pick_time().unwrap()), "%Y-%m-%d %H:%M:%S").expect("Full DateTime string");
                    let message = store_message().unwrap();
                    countdown(message_time);
                    if send_message_to_recipient(choice.number.clone(), message.clone(), &account_number) {
                        println!("\"{}\" sent to {} @ {}", message.blink().bold().blue(), choice.number.red().italic(), message_time.to_string().italic().underline().bright_purple());
                    }
                } else {
                    println!("{}", "Canceled!".bold().red());
                }
            }
            println!("");
        },
        Err(_) => println!("You did not select a valid option!"),
    }
}

fn get_recipients(input: &str) -> Vec<Contact> {
    let mut contacts: Vec<Contact> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let start_bytes = line.find("Profile name: ").unwrap();
        let end_bytes = line.find("Username: ").unwrap();
        let value_bytes = "Profile name: ".len();
        let full_profile_name = &line[(start_bytes + value_bytes)..end_bytes];
        let contact_field_values = line.replace("Number: ", "").replace("Name: ", "").replace("Profile name: ", "").replace("Username:", "").replace("Color: ", "").replace("Blocked: ", "").replace("Message expiration: ", "");
        let fields: Vec<_> = contact_field_values.trim().split(' ').collect();

        let blocked = contact_field_values.contains("true");
        // Very clunky but works
        let mut expiration: String;
        if fields.last().unwrap().ends_with("s") {
            expiration = fields.last().unwrap().to_string();
            expiration.pop();
        } else {
            expiration = "disabled".to_string();
        }

        let message_expiration = match expiration.as_str() {
            "disabled" => MessageExpiration::Disabled,
            value => {
                if let Ok(seconds) = value.parse::<u32>() {
                    MessageExpiration::TimeInSeconds(seconds)
                } else {
                    MessageExpiration::Disabled // Chuck unexpected values as Disabled
                }
            }
        };

        let contact = Contact {
            id: index as u16,
            number: format!("{}", fields[0].trim()),
            name: format!("{}", fields[1].trim()),
            profile_name: full_profile_name.trim().to_string(),
            blocked,
            message_expiration,
            ..Default::default()
        };
        contacts.push(contact);
    }
    contacts
}

fn pick_date() -> Result<NaiveDate, InquireError> {
    let date = DateSelect::new("Date on which to send the message:")
        .with_validator(|d: NaiveDate| {
            let yesterday = (Local::now() - Duration::days(1)).naive_local().date();
            if d.le(&yesterday) {
                Ok(Validation::Invalid("Date must be either a future date or today!".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt();

    match date {
        Err(_) => panic!("Error picking date!"),
        Ok(_) => (),
    }
    date
}

fn pick_time() -> Result<NaiveTime, InquireError> {
    let time = CustomType::<NaiveTime>::new("Time of day on witch to send the message:")
        .with_default(NaiveTime::parse_from_str("12:00", "%H:%M").expect("12:00 as default"))
        .with_parser(&|i| NaiveTime::parse_from_str(i, "%H:%M").map_err(|_| ()))
        .with_help_message("Enter time in the format of <%H:%M>")
        .with_error_message("Entered time not in the format: <%H:%M> !")
        .with_formatter(&|i| i.to_string())
        .prompt();
    time
}

fn store_message() -> Result<String, InquireError> {
    // TODO: add support for new lines, '\n' or "\\n", "\r\n" don't seem to work
    let message = Text::new("Type the message you want sent:")
        .with_help_message("Signal formating might work, new lines don't tho...")
        .with_validator(required!("Cannot send an empty message!"))
        .prompt();
    message
}

fn system_time_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub(crate) fn format_time_from_seconds(seconds: u64) -> String {
    let mut days: u64 = 0;
    let mut hours: u64 = 0;
    let mut minutes: u64 = 0;
    let mut seconds_remaining: u64 = seconds;
    if seconds_remaining / 86400 >= 1 {
        days = seconds_remaining / 86400;
        seconds_remaining %= 86400;
    }
    if seconds_remaining / 3600 >= 1 {
        hours = seconds_remaining / 3600;
        seconds_remaining %= 3600;
    }
    if seconds_remaining / 60 >= 1 {
        minutes = seconds_remaining / 60;
        seconds_remaining %= 60;
    }
    if days == 1 {
        return format!("{0} day, {1:>02}:{2:>02}:{3:>02}", days, hours, minutes, seconds_remaining)
    } else if days > 1 {
       return format!("{0} days, {1:>02}:{2:>02}:{3:>02}", days, hours, minutes, seconds_remaining)
    } else {
        return format!("{0:>02}:{1:>02}:{2:>02}", hours, minutes, seconds_remaining)
    }
}

fn send_message_to_self(message: String) -> bool {
    match Command::new("signal-cli").args(["send", "--note-to-self", "-m", &message]).output() {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("`signal-cli` wrote to stderr: {}", stderr);
                false
            }
        }
        Err(e) => {
            println!("Failed to run `signal-cli send` command: {}", e);
            false
        }
    }
}

fn send_message_to_recipient(number: String, message: String, account: &str) -> bool {
    match Command::new("signal-cli").args(["-a", &account, "send", "-m", &message, &number]).output() {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("`signal-cli` wrote to stderr: {}", stderr);
                false
            }
        }
        Err(e) => {
            println!("Failed to run `signal-cli send` command: {}", e);
            false
        }
    }
}

pub(crate) fn get_contact_list() -> Option<String> {
    match Command::new("signal-cli").arg("listContacts").output() {
        Ok(output) => {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).to_string()) 
            } else {
                println!("signal-cli returned err: {}", String::from_utf8_lossy(&output.stderr));
                None
            }
        }
        Err(e) => {
            println!("Failed to run `signal-cli`: {}", e);
            None
        }
    }
}

fn get_account_number(contacts: &str) -> String {
    // Extracts primary device account number // might fail with multiple accounts?
    let mut words = contacts.split_whitespace();
    let _ = words.next();
    if let Some(word) = words.next() {
        word.to_string()
    } else {
        panic!("Couldn't extract OWN number!"); 
    }
}

fn countdown(message_time: NaiveDateTime) {
    let local_message_time = Berlin.from_local_datetime(&message_time).unwrap();
    let u_message_time: u64 = local_message_time.timestamp().try_into().unwrap();
    let current_time = system_time_seconds();
    let wait_time = u_message_time - current_time;
    for i in (0..wait_time).rev() {
        // Add a spinner for the countdown?
        print!("Message will be sent in {} {}", format_time_from_seconds(i).bold().yellow(), "Press Ctrl+C to cancel".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(SystemDuration::from_secs(1));
        print!("{}[1K\r", 27 as char);
    }
}
