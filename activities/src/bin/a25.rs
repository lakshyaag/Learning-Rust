// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calc_perimeter(&self) -> f64;
}

struct Square {
    side: f64,
}

impl Perimeter for Square {
    fn calc_perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("The perimeter is {:?}", shape.calc_perimeter())
}

fn main() {
    let s = Square { side: 40.0 };

    let t = Triangle {
        a: 4.3,
        b: 3.4,
        c: 10.2,
    };

    print_perimeter(s);
    print_perimeter(t);
}
