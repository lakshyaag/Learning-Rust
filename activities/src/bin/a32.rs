// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("../data/mock-data.csv");

struct Data<'a> {
    names: Vec<&'a str>,
    titles: Vec<&'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let data = Data {
        names: data.iter().filter_map(|l| l.split(',').nth(1)).collect(),
        titles: data.iter().filter_map(|l| l.split(',').nth(4)).collect(),
    };

    let zipped_data = data.names.iter().zip(data.titles.iter());

    for (name, title) in zipped_data.take(5) {
        println!("Name: {}, Title: {}", name, title);
    }
}
