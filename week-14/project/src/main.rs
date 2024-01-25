use std::{io::self,io::Read, fs::File};

fn main() {
    let input = io::stdin();
    let mut choice = String::new();

    println!("\nEnter the letter representing your position:");
    println!("Database admin (a)\nManager (m)\nEmployee (e)\nCustomer (c)\nVendor (v)");

    println!("\nYour choice:");
    input.read_line(&mut choice).expect("Could not read");
    let choice:char = choice.trim().parse().expect("Not a valid character");

    if choice == 'a' {
        admin();
    } else if choice == 'm' {
        manager();
    } else if choice == 'e' {
        employee();
    } else if choice == 'c' {
        customer();
    } else if choice == 'v' {
        vendor();
    } else {
        println!("Why do you like to rebel!?");
    }
}

fn admin() {
    let mut file = File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents);
}

fn manager() {
    let mut file = File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents);
}

fn employee() {
    let mut file = File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents);
}

fn customer() {
    let mut file = File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents);
}

fn vendor() {
    let mut file = File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("\n{}", contents);
}