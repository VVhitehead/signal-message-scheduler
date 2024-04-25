use std::fmt;
use colored::Colorize;

use crate::util::format_time_from_seconds;


#[derive(Debug, Default, Clone)]
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
                self.name.bold().green(),
                self.profile_name.underline().cyan(),
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
