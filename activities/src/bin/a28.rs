// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoes(Color);
struct Shirt(Color);
struct Pants(Color);

impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
    fn print_color_shoes(&self) {
        println!("Shoes are {:?}", self.0)
    }
}

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
    fn print_color_shirt(&self) {
        println!("Shirt is {:?}", self.0)
    }
}

impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
    fn print_color_pants(&self) {
        println!("Pants are {:?}", self.0)
    }
}

fn main() {
    let shoes = Shoes::new(Color::Black);
    let shirt = Shirt::new(Color::Custom("Lavender".to_owned()));
    let pants = Pants::new(Color::Gray);

    shoes.print_color_shoes();
    shirt.print_color_shirt();
    pants.print_color_pants();
}
