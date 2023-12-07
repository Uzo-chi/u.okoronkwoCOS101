use std::format;
use std::io::{self, Write};

fn main() {
    let input = io::stdin();
    let mut info: Vec<[String;3]> = Vec::new();

    let mut file = std::fs::File::create("commissioners.txt").expect("Could not create!");
    file.write_all(format!("{:<50} {:<50} {:<20}\n\n", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE").as_bytes()).expect("Could not write");

    loop {
        let mut name = String::new();
        let mut ministry = String::new();
        let mut zone = String::new();

        println!("\nEnter commissioner's name (Enter 'quit' to end):");
        input.read_line(&mut name).expect("Could not read!");
        let name = name.trim();

        if name == "quit" {
            break;
        }

        println!("Enter their ministry:");
        input.read_line(&mut ministry).expect("Could not read!");
        let ministry = ministry.trim();

        println!("Enter their geopolitical zone:");
        input.read_line(&mut zone).expect("Could not read!");
        let zone = zone.trim();

        let commissioner: [String;3] = [name.to_string(), ministry.to_string(), zone.to_string()];
        info.push(commissioner);
    }

    println!("");
    println!("Commissioner Info:");
    for intel in info.iter() {
        println!("{:?}", intel);
        for i in 0..3 {
            if i == 2 {
                file.write_all(format!("{:<20}\n",intel[i]).as_bytes());
            } else {
                file.write_all(format!("{:<50} ",intel[i]).as_bytes());
            }
        }
    }
    println!("\nData written!");
}