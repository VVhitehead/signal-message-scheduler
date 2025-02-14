use std::fmt;
use colored::Colorize;

use crate::util::format_time_from_seconds;
//use crate::util::get_own_accounts;


#[derive(Debug, Clone, Default)]
pub enum MessageExpiration {
        TimeInSeconds(u32),
        #[default] Disabled,
}

#[derive(Debug, Default, Clone)]
pub struct Contact {
    pub id: u16,
    pub account_identifier: String,
    pub number: String,
    pub name: String,
    pub profile_name: String,
    pub blocked: bool,
    pub message_expiration: MessageExpiration,
    pub _color: String // Unimportant field, could maybe use it to color profile names with similar color?
}

#[derive(Debug, Default, Clone)]
pub struct Group {
    pub id: String,
    pub numeric_id: u16,
    pub name: String,
    pub description: String,   
    pub active: bool,
    pub blocked: bool,
    pub members: Vec<String>,
    pub pending_members: Vec<String>,
    pub requesting_members: Vec<String>,
    pub admins: Vec<String>,
    pub banned: Vec<String>,
    pub message_expiration: MessageExpiration,
    pub link: String // https://signal.group/#Cj... // should it be some kind of url type insead of String?
}


// Conversion from u32 to MessageExpiration
impl From<u32> for MessageExpiration {
    fn from(seconds: u32) -> Self {
        MessageExpiration::TimeInSeconds(seconds)
    }
}

// Implement fmt::Display for MessageExpiration
impl fmt::Display for MessageExpiration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageExpiration::TimeInSeconds(seconds) => write!(f, "{}", format_time_from_seconds(*seconds as u64).yellow()),
            MessageExpiration::Disabled => write!(f, "{}", "disabled".to_string().green()),
        }
    }
}

// Implement Display trait for Contact
impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.blocked {
            write!(
                f,
                "{}|{}, blocked: {}, message_expiration: {}",
                self.name.bold().red(),
                self.profile_name.underline().red(),
                self.blocked.to_string().underline().to_uppercase().bold().red(),
                self.message_expiration,
            )
       } else {
            write!(
                f,
                "{}|{}, message_expiration: {}",
                self.name.bold().green(),
                self.profile_name.underline().cyan(),
                self.message_expiration,
            )
        }
    }
}

// Implement Display trait for Group
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.active {
                write!(
                    f,
                    "{}, message_expiration: {}, Description: {}",
                    self.name.bold().green(),
                    self.message_expiration,
                    self.description.italic().blue(),
                    //self.admins.join(" ").green(),
                    //self.members.join(" | ").underline().bright_red(),
                )
        } else {
            write!(
                f,
                "{}, Active: {}, Description: {}",
                self.name.bold().red(),
                self.active.to_string().to_uppercase().bold().red(),
                self.description.underline().bright_red(),
            )
        }
    }
}
