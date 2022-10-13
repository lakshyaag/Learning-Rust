// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

impl Person {
    fn print(&self) {
        // println!("Age: {:?}", self.age);
        println!("Name: {:?}", self.name);
        println!("Favorite color: {:?}", self.fav_color);
    }
}

fn main() {
    let people = vec![
        Person {
            age: 1,
            name: String::from("John"),
            fav_color: "Red".to_owned(),
        },
        Person {
            age: 2,
            name: String::from("Jane"),
            fav_color: "Blue".to_owned(),
        },
        Person {
            age: 22,
            name: String::from("Lakshya"),
            fav_color: "Green".to_owned(),
        },
        Person {
            age: 21,
            name: String::from("Agarwal"),
            fav_color: "Red".to_owned(),
        },
    ];

    for person in people {
        if person.age <= 10 {
            person.print()
        }
    }
}
