use std::io;

fn main() {
    let input = io::stdin();
    let mut size = String::new();
    let mut years: Vec<usize> = Vec::new();
    let mut names: Vec<String> = Vec::new();

    println!("How many people will be comparing?:");
    input.read_line(&mut size).expect("Cannot read");
    let size:usize = size.trim().parse().expect("Not a valid integer");

    for count in 0..size {
        vector_pusher(&mut names, &mut years);
    }

    let max_index = max_getter(&size, &years);
    
    println!("\nThe person with the most programming experience is {} with {} years", names[max_index], years[max_index]);
}

fn vector_pusher(vect1: &mut Vec<String>, vect2: &mut Vec<usize>){
    let input = io::stdin();
    let mut name = String::new();
    let mut year = String::new();

    {
        println!("\nWhat is your name?:");
        input.read_line(&mut name).expect("Could not read");
        let name = name.trim();
        vect1.push(name.to_string());

        println!("How long have you been programming?:");
        input.read_line(&mut year).expect("Could not read");
        let year:usize = year.trim().parse().expect("Not a valid integer");
        vect2.push(year);
    }
}

fn max_getter(n: &usize, vect: &Vec<usize>) -> usize {
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..*n {
        if vect[i] > max {
            max = vect[i];
            max_index = i;
        }
    }
    return max_index;
}