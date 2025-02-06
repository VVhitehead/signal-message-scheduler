use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use inquire::{Select, InquireError, Confirm, Text, validator::Validation, CustomType, DateSelect, required};
//use inquire::formatter::StringFormatter;
use std::cmp::Ordering;
use std::process::Command;
use std::thread;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH, Duration as SystemDuration};
use std::io::{self, Write};
use colored::Colorize;

use crate::contact::{Contact, MessageExpiration, Group};

pub(crate) fn initial_select() -> String {
    let options: Vec<&str> = vec!["Contacts", "Groups"];
    let choice = Select::new("Access your contacts or groups?", options)
        .with_help_message("Contacts are verified Signal users you've added and Groups are a collection of memebers which may or may not be in your Signal contacts.").prompt();

    match choice {
        Ok(choice) => {
            if choice == "Contacts" {
                "contacts".to_string()
            } else if choice == "Groups" {
                "groups".to_string()
            } else {
                panic!("Failed to fetch groups(Maybe you're not a member of any groups?)");
            }
        },
        Err(_) => panic!("Red or blue!"),
    }
}

pub(crate) fn run_group_dialogue(grouplist: String) {
    //let account_number = get_own_accounts().unwrap();
    let groups = get_groups(&grouplist);
    let choice = Select::new("Write and schedule a message for:", groups).prompt();

    match choice {
        Ok(choice) => {
            if !choice.blocked {
                let confirmed = Confirm::new(&(format!("You want to message: {} ({})", choice.name.green().bold(),
                    format!(
                        "{}…", choice.description.get(0..20).unwrap_or(&choice.description)
                    ).blue().italic())))
                    .with_default(true)
                    .prompt()
                    .unwrap();
                if confirmed {
                    let message_time = NaiveDateTime::parse_from_str(&format!("{} {}", pick_date().unwrap(), pick_time().unwrap()), "%Y-%m-%d %H:%M:%S").expect("Full DateTime string");
                    let message = store_message().unwrap();
                    countdown(message_time);
                    if send_message_to_group(choice.id, message.clone()) {
                        println!("\"{}\" sent to group: {} @ {}", message.blink().bold().blue(), choice.name.green().bold(), message_time.to_string().italic().underline().bright_purple());
                    }
                } else {
                   println!("Failed sending to group: {}", choice.name);
                }
            } 
            else {
               eprintln!("You are blocked from the group: {}", choice.name);
            }
            println!();
        }
        Err(_) => panic!("You did not select a valid option!"),
    }
}

pub(crate) fn run_contact_dialogue(list_of_contacts: String) {
    //let _account_number = get_account_number(&list_of_contacts);
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
                    if send_message_to_recipient(choice.account_identifier.clone(), message.clone()) {
                        if !choice.number.is_empty() {
                            println!("\"{}\" sent to {} @ {}", message.blink().bold().blue(), choice.number.red().italic(), message_time.to_string().italic().underline().bright_purple());
                        } else {
                            println!("\"{}\" sent to {} @ {}", message.blink().bold().blue(), choice.account_identifier.red().italic(), message_time.to_string().italic().underline().bright_purple());
                        }
                    }
                } else {
                    println!("{}", "Canceled!".bold().red());
                }
            }
            println!();
        },
        Err(_) => println!("You did not select a valid option!"),
    }
}

fn get_groups(input: &str) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let split_input: Vec<&str> = input.split("\n").collect();
    for (index, line) in split_input.into_iter().enumerate() {
        let id = extract_between(line, "Id: ", " Name: ");
        let active = extract_between(line, "Active: ", " Blocked: ").trim() == "true";
        let blocked = extract_between(line, "Blocked: ", " Members: ").trim() == "true";
        let name = extract_between(line, "Name: ", " Description: ").trim().to_string();
        // Indicate a new line has been replaced for consolidation of precious terminal space and
        // maintaining one line per field whenever the terminal width is sufficient to allow for it 
        let description = extract_between(line, "Description: ", " Active: ").replace('\n', "␤ 🡿 ").to_string(); 
        let members: Vec<_> = extract_between(line, "Members: [", "] Pending members").split(", ").map(|x| x.to_string()).collect();
        let pending_members: Vec<_> = extract_between(line, "Pending members: [", "] Requesting members: ").split(", ").map(|x| x.to_string()).collect();
        let requesting_members: Vec<_> = extract_between(line, "Requesting members: [", "] Admins: ").split(", ").map(|x| x.to_string()).collect();
        let admins: Vec<_> = extract_between(line, "Admins: [", "] Banned: ").split(", ").map(|x| x.to_string()).collect();
        let banned: Vec<_> = extract_between(line, "Banned: [", "] Message expiration: ").split(", ").map(|x| x.to_string()).collect();
        let link = line.split(' ').last().expect("Proper \"https://signal.group/#XXX...\" url or '-' for no link!").to_string();

        let message_expiration = match extract_between(line, "Message expiration: ", "Link: ").as_str() {
            "disabled" => MessageExpiration::Disabled,
            value => {
                if let Ok(seconds) = value.trim_end_matches('s').parse::<u32>() {
                    MessageExpiration::TimeInSeconds(seconds)
                } else {
                    MessageExpiration::Disabled // Chuck unexpected values as Disabled
                }
            }
        };

        let group = Group {
            id,
            numeric_id: index as u16,
            name,
            description,
            active,
            blocked,
            members,
            pending_members,
            requesting_members,
            admins,
            banned,
            message_expiration,
            link,
        };
        groups.push(group);
    }
    groups
}

fn get_recipients(input: &str) -> Vec<Contact> {
    let mut contacts: Vec<Contact> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let number = extract_between(line, "Number: ", " ACI: ");
        let account_identifier = extract_between(line, " ACI: ", " Name: ");
        let name = extract_between(line, " Name: ", " Profile name: ");
        let profile_name = extract_between(line, " Profile name: ", " Username: ");
        let blocked = extract_between(line, " Blocked: ", " Message expiration: ").trim() == "true";

        let message_expiration = match line.split(" Message expiration: ").last().expect("string ‟disabled‟, or a string representing the number in seconds with a trailing 's'(‟2419200s‟)") {
            "disabled" => MessageExpiration::Disabled,
            value => {
                if let Ok(seconds) = value.trim_matches('s').parse::<u32>() {
                    MessageExpiration::TimeInSeconds(seconds)
                } else {
                    MessageExpiration::Disabled // Chuck unexpected values as Disabled
                }
            }
        };

        let contact = Contact {
            id: index as u16,
            number,
            account_identifier,
            name,
            profile_name,
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
    let time = CustomType::<NaiveTime>::new("Time of day on which to send the message:")
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
    // Formater does not seem to work it wokrs in the terminal but signal doesn't convert the new
    // lines but leaves them as raw `\n` cuz I don't wrap the message in "" duh
    //let formatter: StringFormatter = &|input| {
    //    format!("\"{}\"", input.replace("\\n", "\n"))
    //};
    let message = Text::new("Type the message you want sent:")
        .with_help_message("Signals(`$ man signal-cli | grep style`) formating might work, new lines don't tho...")
        .with_validator(required!("Cannot send an empty message!"))
        //.with_formatter(formatter)
        .prompt();
    message
}

fn system_time_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

//Return result as String or empty an empty String if not found
fn extract_between<'a>(source: &'a str, start: &'a str, end: &'a str) -> String {
    let start_position = source.find(start);

    if start_position.is_some() {
        let start_position = start_position.unwrap() + start.len();
        let source = &source[start_position..];
        let end_position = source.find(end).unwrap_or_default();
        return source[..end_position].to_string();
    }
    "".to_string()
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
    match days.cmp(&1) {
        Ordering::Less => format!("{0:>02}:{1:>02}:{2:>02}", hours, minutes, seconds_remaining),
        Ordering::Equal => format!("{0} day, {1:>02}:{2:>02}:{3:>02}", days, hours, minutes, seconds_remaining),
        Ordering::Greater => format!("{0} days, {1:>02}:{2:>02}:{3:>02}", days, hours, minutes, seconds_remaining)
    }
}

fn send_message_to_self(message: String) -> bool {
    match Command::new("signal-cli").args(["send", "--note-to-self", "--notify-self", "-m", &message]).output() {
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

fn send_message_to_recipient(account_id: String, message: String) -> bool {
    match Command::new("signal-cli").args(["send", "-m", &message, &account_id]).output() {
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

fn send_message_to_group(id: String, message: String) -> bool {
    match Command::new("signal-cli").args(["send", "-g", &id, "-m", &message]).output() {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("`singal-cli` wrote to stderr: {}", stderr);
                false
            }
        }
        Err(e) => {
            println!("Failed to run `signal-cli send -g` command: {}", e);
            false
        }
    }
}

pub(crate) fn get_contact_list(option: &str) -> Option<String> {
    if option == "contacts" {
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
    } else {
        panic!("Feck Groups not implemented yet!");
    }
}

pub(crate) fn get_group_list() -> Option<String> {
    match Command::new("signal-cli").args(["listGroups", "-d"]).output() {
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

#[allow(dead_code)] // Makes an additional call adding unnecessary wait time unless dealing with multiple Signal accounts
// Change to `Option<Vec<String>>` to support multiple accounts
pub(crate) fn get_own_accounts() -> Option<String> {
    match Command::new("signal-cli").arg("listAccounts").output() {
        Ok(output) => {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).replace("Number: ", "").to_string()) 
            } else {
                println!("signal-cli returned err: {}", String::from_utf8_lossy(&output.stderr));
                None
            }
        }
        Err(e) => {
            println!("Failed to run `signal-cli listAccounts`: {}", e);
            None
        }
    }
}

#[allow(dead_code)]
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
    let local_message_time = Local.from_local_datetime(&message_time).unwrap();
    let u_message_time: u64 = local_message_time.timestamp().try_into().unwrap();
    let current_time = system_time_seconds();
    let wait_time = u_message_time - current_time;
    for i in (0..wait_time).rev() {
        print!("Message will be sent in {} {}", format_time_from_seconds(i).bold().yellow(), "Press Ctrl+C to cancel".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(SystemDuration::from_secs(1));
        print!("\x1b[1K\r");
    }
}
