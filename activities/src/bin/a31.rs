// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

use std::io;

trait Material {
    fn cost(&self) -> f64;
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost(&self) -> f64 {
        self.0 * 10.0
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost(&self) -> f64 {
        self.0 * 15.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost(&self) -> f64 {
        self.0 * 20.0
    }
}

fn get_user_input() -> io::Result<f64> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().parse().expect("Please enter a number!"))
}

fn total_cost(materials: &Vec<Box<dyn Material>>) -> f64 {
    materials.iter().map(|mat| mat.cost()).sum()
}

fn main() {
    println!("Enter size, in square meters: ");
    let sqmtr = get_user_input().expect("Please enter a number!");

    let carpet = Box::new(Carpet(sqmtr));
    let tile = Box::new(Tile(sqmtr));
    let wood = Box::new(Wood(sqmtr));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    let cost = total_cost(&materials);
    println!("Total cost is: {:?}", cost);
}
