use std::println;
use colored::Colorize;
use spinners::{Spinner, Spinners};

pub mod util;
pub mod contact;


fn main() {
    println!("    -- Message scheduler for {} Messenger --\n\n","Signal".bold().blue());

    let choice = util::initial_select(); 
    if choice == "contacts" {
        let mut sp = Spinner::new(Spinners::Point, String::new());
        let contacts = util::get_contact_list(&choice);
        sp.stop_with_symbol("\x1b[2K\x1b");
        util::run_contact_dialogue(contacts.unwrap().to_string());
    } else if choice == "groups" {
        let mut sp = Spinner::new(Spinners::Point, String::new());
        let groups = util::get_group_list();
        sp.stop_with_symbol("\x1b[2K\x1b");
        util::run_group_dialogue(groups.unwrap());
    } else {
        panic!("Neither option selected!");
    }
}
