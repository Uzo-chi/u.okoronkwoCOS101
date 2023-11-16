use std::io;

fn main() {
    let mut count = 0;
    let input = io::stdin();

    while count < 150 {
        let mut rep = String::new();
        let mut name = String::new();
        let mut year = String::new();
        let mut cgpa = String::new();
        let mut email = String::new();
        let mut dept = String::new();
        let mut state = String::new();

        println!("Are you a Class Representative? (Enter y/n):");
        input.read_line(&mut rep).expect("This might just not be a string!");
        let rep:char = rep.trim().parse().expect("I don't think this is a character!");

        println!("What level are you in? (100 - 500):");
        input.read_line(&mut year).expect("This might just not be a string!");
        let year:i32 = year.trim().parse().expect("I don't think this is an integer!");

        println!("What is your CGPA? (please do not lie):");
        input.read_line(&mut cgpa).expect("This might just not be a string!");
        let cgpa:f64 = cgpa.trim().parse().expect("Make sure this is a float (e.g. 4.0)!");

        println!("What is your name?:");
        input.read_line(&mut name).expect("This might just not be a string!");
        let name = name.trim();

        println!("What is your email?:");
        input.read_line(&mut email).expect("This might just not be a string!");
        let email = email.trim();

        println!("Which department are you in?:");
        input.read_line(&mut dept).expect("This might just not be a string!");
        let dept = dept.trim();

        println!("What is your state of origin?:");
        input.read_line(&mut state).expect("This might just not be a string!");
        let state = state.trim();

        if rep == 'y' || rep == 'n' {
            if rep == 'y' && year > 100 && cgpa > 4.0 {
                println!("\nName: {}\nEmail: {}\nDepartment: {}\nState of Origin: {}\nYou can vote!\n", name, email, dept, state);
            } else {
                println!("Sorry, you are not eligible to vote!");
            }
        } else {
            println!("Quit playing games with me!");
        }

        count += 1;
    }
}
