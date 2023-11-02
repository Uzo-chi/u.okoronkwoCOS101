use std::io;

fn main() {
    let input = io::stdin();
    let mut distance = String::new();
    let mut time = String::new();

    println!("Enter the distance (in miles):");
    input.read_line(&mut distance).expect("This string is invalid");
    let d_m:f64 = distance.trim().parse().expect("Is this a number?");

    println!("Enter the time taken:");
    input.read_line(&mut time).expect("This string is invalid");
    let t:f64 = time.trim().parse().expect("Is this a number?");

    let d_k:f64 = d_m * 1.609;
    let s = d_k / t;

    println!("The speed of the car is {}km/h", s);
}
