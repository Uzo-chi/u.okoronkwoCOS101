use std::io;

fn main() {
    let input = io::stdin();
    let mut size = String::new();
    let mut years: Vec<usize> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut languages: Vec<String> = Vec::new();

    println!("How many people will be comparing?:");
    input.read_line(&mut size).expect("Cannot read");
    let size:usize = size.trim().parse().expect("Not a valid integer");

    for count in 0..size {
        vector_pusher(&mut names, &mut years, &mut languages);
    }

    let max_index = max_getter(&size, &years);
    
    println!("\n{} is the most qualified with {} years of experience in {} programming language",
        names[max_index], years[max_index], languages[max_index]);
}

fn vector_pusher(vect1: &mut Vec<String>, vect2: &mut Vec<usize>, vect3: &mut Vec<String>){
    let input = io::stdin();
    let mut name = String::new();
    let mut year = String::new();
    let mut language = String::new();

    {
        println!("\nWhat is your name?:");
        input.read_line(&mut name).expect("Could not read");
        let name = name.trim();
        vect1.push(name.to_string());

        println!("\nWhat language do you specialize in?(only one):");
        input.read_line(&mut language).expect("Could not read");
        let language = language.trim();
        vect3.push(language.to_string());

        println!("How long have you been programming in that language?(in years):");
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