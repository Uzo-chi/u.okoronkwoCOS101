use std::io;

fn main() {
    let input = io::stdin();
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced (y/n):");
    input.read_line(&mut experience).expect("This might not be a string");
    let experience = experience.trim();

    println!("How old are you?:");
    input.read_line(&mut age).expect("This might not be a string");
    let age:i64 = age.trim().parse().expect("This is not a correct integer");

   if experience == "y" {

    if age >= 40 {

        let i:i64 = 1_560_000; 
        println!("Your incentive is {}", i);
    } else if age >= 30 {

        let i:i64 = 1_480_000;
        println!("Your incentive is {}", i);
    } else if age <= 29 {

        let i:i64 = 1_300_000;
        println!("Your incentive is {}", i);
    }
   } else if experience == "n" {
    let i:i64 = 100_000;
    println!("Your incentive is {}", i);
   } else {
    println!("You didn't y/n, did you?");
   }
}
