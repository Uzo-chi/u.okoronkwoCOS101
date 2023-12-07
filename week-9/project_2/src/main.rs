use std::io::{self, Write};

fn main() {
    let input = io::stdin();
    let mut info: Vec<[String;4]> = Vec::new();

    let mut file = std::fs::File::create("students.txt").expect("Could not create!");
    file.write_all("STUDENT NAME\t\tMATRIC. NUMBER\t\tDEPARTMENT\t\tLEVEL\n\n".as_bytes()).expect("Could not write");

    loop {
        let mut name = String::new();
        let mut matric = String::new();
        let mut dept = String::new();
        let mut level = String::new();

        println!("\nEnter student's name (Enter 'quit' to end):");
        input.read_line(&mut name).expect("Could not read!");
        let name = name.trim();

        if name == "quit" {
            break;
        }

        println!("Enter student's matriculation number:");
        input.read_line(&mut matric).expect("Could not read!");
        let matric = matric.trim();

        println!("Enter student's department:");
        input.read_line(&mut dept).expect("Could not read!");
        let dept = dept.trim();

        println!("Enter student's level:");
        input.read_line(&mut level).expect("Could not read!");
        let level = level.trim();

        let student: [String;4] = [name.to_string(), matric.to_string(), dept.to_string(), level.to_string()];
        info.push(student);
    }

    println!("");
    println!("Student Information:");
    for intel in info.iter() {
        println!("{:?}", intel);
        for i in 0..4 {
            if i == 3 {
                file.write_all(intel[i].as_bytes());
                file.write_all("\n".as_bytes());
            } else {
                file.write_all(intel[i].as_bytes());
                file.write_all("\t\t".as_bytes());
            }
        }
    }
    println!("\nData written!");
}