// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

#[derive(Debug)]
struct Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    body: T,
    color: U,
}

impl<T, U> Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    fn new(body: T, color: U) -> Self {
        Self {
            body: body,
            color: color,
        }
    }
}
#[derive(Debug)]
struct Car;
#[derive(Debug)]
struct Truck;

impl Body for Car {}
impl Body for Truck {}

#[derive(Debug)]
struct Red;
#[derive(Debug)]
struct Black;

impl Color for Red {}
impl Color for Black {}

fn main() {
    let red_car = Vehicle::new(Car, Red);
    let black_truck = Vehicle::new(Truck, Black);

    println!("{:?}", red_car);
    println!("{:?}", black_truck);
}
