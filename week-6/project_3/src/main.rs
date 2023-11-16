use std::io;

fn main() {
    let mut n = String::new();
    let input = io::stdin();

    println!("How many numbers would you like to see:");
    input.read_line(&mut n).expect("Not a string!");
    let n:u64 = n.trim().parse().expect("Not a real integer!");

    println!("");

    for a in 1..n+1 {
        for b in 1..n+1 {
            print!("{}\t", a*b);
        }
        println!("");
    }
}
