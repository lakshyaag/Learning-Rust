// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("chairs", 5);
    stock.insert("beds", 3);
    stock.insert("tables", 2);
    stock.insert("couches", 0);

    let mut total_stock = 0;
    
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        match qty {
            0 => println!("{:?} is out of stock", item),
            _ => println!("{:?} has {:?} quantity", item, qty),
        }
    }

    println!("Total quantity is {:?}", total_stock)
}
