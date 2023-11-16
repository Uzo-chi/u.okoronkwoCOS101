use std::io;

fn main() {
    let input = io::stdin();
    for x in 0..500 {
        let mut name = String::new();
        let mut papers = String::new();

        println!("Name:");
        input.read_line(&mut name).expect("Not a string!");
        let name = name.trim();

        println!("Number of papers published:");
        input.read_line(&mut papers).expect("Not a string!");
        let papers:u32 = papers.trim().parse().expect("This isn't a real integer");

        if papers >= 3 && papers <= 5 {
            println!("Name: {}\nIncentive: N500,000\n", name);
        } else if papers > 5 && papers < 10 {
            println!("Name: {}\nIncentive: N800,000\n", name);
        } else if papers > 10 {
            println!("Name: {}\nIncentive: N1,000,000\n", name);
        } else {
            println!("Name: {}\nIncentive: N100,000\n", name);
        }
    }
}
