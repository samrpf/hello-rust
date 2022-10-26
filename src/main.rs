// my very good program
// in Rust!
// By samrpf on GitHub

extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };
use std::time::Duration;
use std::thread::sleep;

// main function
fn main() {
    println!("Hello, world!");
    println!("Welcome to the Employee Console!");
    println!(); // print newline

    // create employees
    let employee1 = Employee {
        id: 37281,
        name: String::from("Bean")
    };
    let employee2 = Employee {
        id: 37282,
        name: String::from("Other Bean")
    };
    let employee3 = Employee {
        id: 37283,
        name: String::from("Another Bean")
    };
    let employee4 = Employee {
        id: 37284,
        name: String::from("Another Another Bean")
    };

    // put employees in a bunch
    let bunch_of_employees = [employee1, employee2, employee3, employee4];

    // Make pause info stuffs
    let duration_to_sleep = Duration::from_secs(3);

    // Print employee information
    for employee in bunch_of_employees {
        println!("NEW EMPLOYEE!");
        println!("ID: {}", employee.id);
        println!("Name: {}", employee.name);
        println!(); // print newline after
        sleep(duration_to_sleep); // then sleep for a few seconds
    }

    // Done with employees
    println!("No new employees are appearing.");
    println!("You can now see your employees.");

    // create message
    let message =  b"Hello fellow bean friend";
    let width = 24;

    // create writer
    let mut writer = BufWriter::new(stdout());

    // start loop
    for _n in 1..3 {
        say(message, width, &mut writer).unwrap();
    }
}

// create Employee struct
struct Employee {
    id: i32,
    name: String
}
