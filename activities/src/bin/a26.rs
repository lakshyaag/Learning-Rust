// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let local_time: DateTime<Local> = Local::now();
    println!("The current time is: {:?}", local_time.format("%d-%m-%Y %H:%M:%S").to_string())
}
