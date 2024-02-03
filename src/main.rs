use std::println;
use colored::Colorize;

pub mod util;
pub mod contact;


fn main() {
    println!("    -- Message scheduler for {} Messenger --\n\n","Signal".bold().blue());
    
    let contacts = util::get_contact_list().unwrap();
    
    util::run_dialogue(contacts.to_string());
}
