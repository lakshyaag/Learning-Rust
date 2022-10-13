// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Strawberry,
    Mango,
    Apple,
    Orange,
}

struct DrinkInfo {
    flavor: Flavor,
    volume: f64,
}

fn print_drink_info(info: DrinkInfo) {
    match info.flavor {
        Flavor::Strawberry => println!("The flavor is Strawberry."),
        Flavor::Mango => println!("The flavor is Mango."),
        Flavor::Apple => println!("The flavor is Apple."),
        Flavor::Orange => println!("The flavor is Orange."),
    };

    println!("The volume is {:?}oz.", info.volume)
}

fn main() {
    let drink_info1 = DrinkInfo {
        flavor: Flavor::Strawberry,
        volume: 8.5,
    };

    let drink_info2 = DrinkInfo {
        flavor: Flavor::Mango,
        volume: 19.32,
    };

    print_drink_info(drink_info1);
    print_drink_info(drink_info2)
}
