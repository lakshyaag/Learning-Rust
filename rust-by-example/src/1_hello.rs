// use std::fmt;

// struct Person {
//     name: String,
//     age: u8,
// }

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{name} is {age} years old",
//             name = self.name,
//             age = self.age
//         )
//     }
// }

// fn main() {
//     println!("Hello, world!");
//     println!("I'm a Rustacean! 🦀");

//     let person = Person {
//         name: String::from("Lakshya"),
//         age: 22,
//     };

//     println!("{person:#}");
// }

// use std::fmt::{self, Formatter, Result}; // Import `fmt`

// // A structure holding two numbers. `Debug` will be derived so the results can
// // be contrasted with `Display`.
// #[derive(Debug)]
// struct MinMax(i64, i64);

// // Implement `Display` for `MinMax`.
// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// // Define a structure where the fields are nameable for comparison.
// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// // Similarly, implement `Display` for `Point2D`.
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Customize so only `x` and `y` are denoted.
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// #[derive(Debug)]
// struct Complex {
//     real: f64,
//     imag: f64,
// }

// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{} + {}i", self.real, self.imag)
//     }
// }

// fn main() {
//     let minmax = MinMax(0, 14);

//     println!("Compare structures:");
//     println!("Display: {}", minmax);
//     println!("Debug: {:?}", minmax);

//     let big_range = MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!(
//         "The big range is {big} and the small is {small}",
//         small = small_range,
//         big = big_range
//     );

//     let point = Point2D { x: 3.3, y: 7.2 };

//     println!("Compare points:");
//     println!("Display: {}", point);
//     println!("Debug: {:?}", point);

//     // Error. Both `Debug` and `Display` were implemented, but `{:b}`
//     // requires `fmt::Binary` to be implemented. This will not work.
//     // println!("What does Point2D look like in binary: {:b}?", point);

//     let complex = Complex {
//         real: 3.3,
//         imag: 7.2,
//     };

//     println!("Compare complex:");
//     println!("Display: {}", complex);
//     println!("Debug: {:?}", complex);
// }

use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
