use std::io;

fn main() {
    let input = io::stdin();
    let mut total:f64 = 0.0;

    println!("\t\tMenu\t\t\tPrice");
    println!("P = Poundo Yam/Edinkaiko Soup\t       -N3,200");
    println!("F = Fried Rice & Chicken\t       -N3,000");
    println!("A = Amala & Ewedu Soup\t\t       -N2,500");
    println!("E = Eba & Egusi Soup\t\t       -N2,000");
    println!("W = White Rice & Stew\t\t       -N2,500");

    println!("Enter letter of food you would like to order (q to quit):");
    loop {
        let mut food = String::new();
        input.read_line(&mut food).expect("Not a string");
        let food = food.trim();

        if food == "P" {
            total += 3200.0;
        } else if food == "F" {
            total += 3000.0;
        } else if food == "A" {
            total += 2500.0;
        } else if food == "E" {
            total += 2000.0;
        } else if food == "W" {
            total += 2500.0;
        } else if food == "q" {
            break;
        }else {
            println!("Sorry, we don't serve that here!");
            continue;
        }
    }
    if total > 10_000.0 {
        let new_total:f64 = total - ((5.0 / 100.0) * total);
        println!("Total is over N10,000, 5% discount: N{}", new_total);
    } else {
        println!("Total: N{}", total);
    }
}