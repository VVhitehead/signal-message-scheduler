use std::println;
use colored::Colorize;
use spinners::{Spinner, Spinners};

pub mod util;
pub mod contact;


fn main() {
    println!("    -- Message scheduler for {} Messenger --\n\n","Signal".bold().blue());

    let mut sp = Spinner::new(Spinners::Point, String::new());   
    let contacts = util::get_contact_list().unwrap();
    sp.stop_with_symbol("\x1b[2K\x1b");
    
    util::run_dialogue(contacts.to_string());
}
