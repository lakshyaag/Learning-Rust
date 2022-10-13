// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => println!("The box color is Red"),
            Color::Green => println!("The box color is Green"),
            Color::Blue => println!("The box color is Blue"),
        };
    }
}

struct BoxDimensions {
    length: i32,
    width: i32,
    height: i32,
}

impl BoxDimensions {
    fn print_dimensions(&self) {
        println!(
            "The box dimensions are {:?} x {:?} x {:?}",
            self.length, self.width, self.height,
        );
    }
}

struct ShippingBox {
    dimensions: BoxDimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: BoxDimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(&self) {
        self.color.print_color();
        self.dimensions.print_dimensions();
        println!("The box weight is {:?}", self.weight);
    }
}

fn main() {
    let shipping_dimensions = BoxDimensions {
        length: 5,
        width: 6,
        height: 10,
    };

    let shipping = ShippingBox::new(shipping_dimensions, 45.5, Color::Blue);
    shipping.print_box();
}
