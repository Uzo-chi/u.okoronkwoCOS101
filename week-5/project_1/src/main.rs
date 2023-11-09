use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let input = io::stdin();

    println!("Value of a:");
    input.read_line(&mut a).expect("This string is invalid");
    let a:f64 = a.trim().parse().expect("Is this a number?");

    println!("Value of b:");
    input.read_line(&mut b).expect("This string is invalid");
    let b:f64 = b.trim().parse().expect("Is this a number?");

    println!("Value of c:");
    input.read_line(&mut c).expect("This string is invalid");
    let c:f64 = c.trim().parse().expect("Is this a number?");

    let d = (b.powf(2.0)) - (4.0 * a * c);

    if d > 0.0 {
        let q_1 = ((0.0 - b) + d.sqrt()) / (2.0 * a);
        let q_2 = ((0.0 - b) - d.sqrt()) / (2.0 * a);

        println!("The roots of the equation are {} and {}.", q_1, q_2);
    } else if d == 0.0 {
        let q = (0.0 - b) / (2.0 * a);

        println!("The root of the equation is {q}.");
    } else {
        println!("There are no real roots of the equation.");
    }
}
