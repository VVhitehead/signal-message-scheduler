use std::println;
use colored::Colorize;
use spinners::{Spinner, Spinners};

pub mod util;
pub mod contact;


fn main() {
    println!("    -- Message scheduler for {} Messenger --\n\n","Signal".bold().blue());

    let mut sp = Spinner::new(Spinners::Point, "Fetching contacts".into());   
    let contacts = util::get_contact_list().unwrap();
    sp.stop();
    print!("\x1b[2K\r");
    
    util::run_dialogue(contacts.to_string());
}
