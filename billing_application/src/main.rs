// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::{io, collections::HashMap};

fn get_user_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your input again.")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_user_input_number() -> Option<f64> {
    loop {
        let input = match get_user_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None
        }

        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number: "),
        }
    }
}

mod menu {
    use crate::{get_user_input, Bill, Bills, get_user_input_number};

    pub fn add(bills: &mut Bills) {
        println!("Enter name of bill: ");
        let bill_name = get_user_input().expect("Invalid user input");
        println!("Enter bill amount: ");
        let bill_amount = get_user_input_number().expect("Invalid user input");
        let new_bill = Bill {
            name: bill_name,
            amount: bill_amount
        };

        bills.add(new_bill);
        println!("New bill added!");
    }

    pub fn view(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove(bills: &mut Bills) {
        println!("Current list of bills: ");
        view(&bills);
        println!("Enter name of bill to remove: ");
        let bill_name = get_user_input().expect("Invalid user input");
        if bills.remove(&bill_name.to_lowercase().as_str()) {
            println!("Bill successfully removed!")
        } else {
            println!("Bill does not exist. Removal unsuccessful.")
        }
    }

    pub fn edit(bills: &mut Bills) {
        println!("Current list of bills: ");
        view(&bills);
        println!("Enter name of bill to update: ");
        let bill_name = get_user_input().expect("Invalid user input");
        println!("Enter updated bill amount: ");
        let new_bill_amount = get_user_input_number().expect("Invalid user input");

        if bills.update(&bill_name.to_lowercase().as_str(), new_bill_amount) {
            println!("Bill successfully updated!")
        } else {
            println!("Bill does not exist. Update unsuccessful.")
        }
    }
}

#[derive(Debug)]
enum UserInput {
    Add,
    View,
    Remove,
    Edit,
}

impl UserInput {
    fn new(choice: &str) -> Option<Self> {
        let choice = choice.parse().unwrap();
        match choice {
            1 => Some(Self::Add),
            2 => Some(Self::View),
            3 => Some(Self::Remove),
            4 => Some(Self::Edit),
            _ => None,
        }
    }

    fn show_choices() {
        println!("");

        println!("-- Choose option -- ");
        println!("1. Add bills");
        println!("2. View existing bills");
        println!("3. Remove bills");
        println!("4. Edit existing bills");

        println!("Enter selection: ");

        println!("");
    }
}

#[derive(Debug)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    value: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { value: HashMap::new() }
    }

    fn add(&mut self, bill: Bill) {
        self.value.insert(bill.name.to_string().to_lowercase(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.value.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.value.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.value.get_mut(name) {
            Some(existing_bill) => {
                existing_bill.amount = amount;
                true
            },
            None => false
        }
    }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        UserInput::show_choices();
        let input = get_user_input().expect("No data entered");
        match UserInput::new(input.as_str()) {
            Some(UserInput::Add) => menu::add(&mut bills),
            Some(UserInput::View) => menu::view(&bills),
            Some(UserInput::Remove) => menu::remove(&mut bills),
            Some(UserInput::Edit) => menu::edit(&mut bills),
            None => return,
        }
    }
}
