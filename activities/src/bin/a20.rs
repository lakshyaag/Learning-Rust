// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum Power {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl Power {
    fn new(state: &str) -> Option<Power> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(Power::Off),
            "sleep" => Some(Power::Sleep),
            "reboot" => Some(Power::Reboot),
            "shutdown" => Some(Power::Shutdown),
            "hibernate" => Some(Power::Hibernate),
            _ => None,
        }
    }

    fn print_state(&self) {
        match self {
            Self::Off => println!("turning off"),
            Self::Sleep => println!("sleeping"),
            Self::Reboot => println!("rebooting"),
            Self::Shutdown => println!("shutting down"),
            Self::Hibernate => println!("hibernating"),
        }
    }
}

fn get_user_input() -> io::Result<String> {
    println!("Enter: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() {

    let user_input = get_user_input().unwrap();
    let power_state = Power::new(&user_input).unwrap();
    power_state.print_state();
    
}
