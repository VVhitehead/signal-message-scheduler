use std::println;
use colored::Colorize;

pub mod util;
pub mod contact;


fn main() {
    println!("    -- Message scheduler for {} Messenger --\n\n","Signal".bold().blue());
    

    let dummy_contacts: &str = "Number: +18404973349 Name:  Profile name: OwnAccount Username:  Color: ultramarine Blocked: false Message expiration: 1209600s
Number: +15714477955 Name: Angel Profile name: BrittaAngel Username:  Color: ultramarine Blocked: false Message expiration: disabled
Number: +16337882179 Name: Haley Profile name: voltanall03 Username:  Color: ultramarine Blocked: false Message expiration: disabled
Number: +12408191628 Name:  Profile name:  Username:  Color: ultramarine Blocked: false Message expiration: disabled
Number: +15753726095 Name: Michael Profile name: Griffith Username:  Color: ultramarine Blocked: false Message expiration: disabled
Number: +19379294080 Name: rosepetle Profile name: rosepetle Username:  Color: ultramarine Blocked: false Message expiration: 2419200s
Number: +17158478329 Name: Kevin Profile name: StarWarsApplePie Username:  Color: ultramarine Blocked: false Message expiration: 2419200s
Number: +12917095732 Name: ALEX Profile name: Alex Thomas Username:  Color: ultramarine Blocked: false Message expiration: disabled
Number: +12225657180 Name: Natalie Profile name: natalie Username:  Color: ultramarine Blocked: true Message expiration: disabled";
    let _contacts = util::get_contact_list().unwrap();
    
    util::run_dialogue(dummy_contacts.to_string());
}
