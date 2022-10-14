// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct LockerAssignment {
    name: String,
    locker_number: Option<i32>,
}

fn main() {
    let students_lockers = vec![
        LockerAssignment {
            name: "Lakshya".to_owned(),
            locker_number: Some(12),
        },
        LockerAssignment {
            name: "Agarwal".to_owned(),
            locker_number: None,
        },
    ];

    for student in students_lockers {
        match student.locker_number {
            Some(number) => println!("{:?} has locker {:?}", student.name, number),
            None => println!("No locker for {:?}", student.name),
        }
    }
}
