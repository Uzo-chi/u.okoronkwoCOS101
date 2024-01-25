use std::format;
use std::io::{self, Write};

struct Company {
    name: String,
    year: usize,
    shares: f64,
    liabilities: f64
}

impl Company {
    fn leverage(&self) -> f64 {
        self.liabilities / self.shares
    }

    fn p_leverage(&self) -> f64 {
        self.leverage() * 100.0
    }
}

fn main() {
    let mut login = false;
    let input = io::stdin();

    let mut file1 = std::fs::File::create("company_info.txt").expect("Could not create!");
    let mut file2 = std::fs::File::create("shares.txt").expect("Could not create!");
    let mut file3 = std::fs::File::create("liabilities.txt").expect("Could not create!");

    let cadbury = Company {name: "Cadbury Nigeria Plc".to_string(), year: 1965, shares: 15_000_000.0, liabilities: 5_500_000.0};
    let champion = Company {name: "Champion Breweries Plc".to_string(), year: 1974, shares: 25_000_000.0, liabilities: 8_000_000.0};
    let dangote = Company {name: "Dangote Sugar Refinery Plc".to_string(), year: 1970, shares: 18_000_000.0, liabilities: 10_000_000.0};
    let flour_mill = Company {name: "Flour Mills Nigeria Plc".to_string(), year: 1960, shares: 32_000_000.0, liabilities: 4_000_000.0};
    let nestle = Company {name: "Nestle Nigeria Plc".to_string(), year: 1961, shares: 8_000_000.0, liabilities: 1_500_000.0};
    let unilever = Company {name: "Unilever Nigeria Plc".to_string(), year: 1923, shares: 37_000_000.0, liabilities: 11_000_000.0};
    let honeywell = Company {name: "Honeywell Nigeria Plc".to_string(), year: 1906, shares: 34_000_000.0, liabilities: 9_000_000.0};
    let nig_brew = Company {name: "Nigerian Breweries Plc".to_string(), year: 1946, shares: 30_000_000.0, liabilities: 12_000_000.0};

    let companies: Vec<Company> = vec![cadbury, champion, dangote, flour_mill, nestle, unilever, honeywell, nig_brew];
    let usernames: Vec<&str> = vec!["Cadb", "Cham", "Dang", "Flou", "Nest", "Unil", "Hone", "Nige"];

    let mut username = String::new();
    let mut password = String::new();

    println!("\nWelcome!\nPlease enter your username and set password to access services");
    println!("Criteria for password:\nOnly letters or numbers, No uppercase, minimum length of 3 characters and maximum of 8");
    println!("\nUsername: ");
    input.read_line(&mut username).expect("Not a valid string!");
    let username = username.trim();
    if usernames.contains(&username) {
        println!("\nPassword: ");
        input.read_line(&mut password).expect("Not a valid string!");
        let password = password.trim();
        if password.chars().all(char::is_alphanumeric) {
            for i in password.chars() {
                if i.is_uppercase() {
                    println!("\nInvalid password!");
                    break;
                } else if !(i.is_uppercase()) && (password.len() >= 3 && password.len() <= 8) {
                    login = true;
                    println!("\nSuccessfully logged in!");
                    break;
                } else {
                    println!("\nInvalid password!");
                    break;
                }
            }
        } else {
            println!("\nInvalid password!")
        }
    } else {
        println!("No valid username - {username} - in database!");
    }

    if login == true {
        file3.write_all(format!("{:<30} | {:<35}", "COMPANY", "5% OF PERCENTAGE LEVERAGE").as_bytes()).expect("Could not write!");
        file2.write_all(format!("{:<30} | {:<35}", "COMPANY", "MULTIPLE OF PERCENTAGE LEVERAGES").as_bytes()).expect("Could not write!");
        file1.write_all(format!("{:<30} | {:<25} | {:<25} | {:<25} | {:<21}",
            "COMPANY", "DATE FOUNDED", "COMPANY SHARES", "COMPANY LIABILITIES", "PERCENTAGE LEVERAGES").as_bytes()).expect("Could not write!");

        for c in companies {
            file1.write_all(format!("\n{:<30} | {:<25} | {:<25} | {:<25} | {:<21.2}",
                c.name, c.year, c.shares, c.liabilities, c.p_leverage()).as_bytes()).expect("Could not write!");
            
            if c.shares > 20_000_000.0 {
                let share_multiple = c.leverage() * c.shares;
                let allowed_debt = c.liabilities + share_multiple;

                file2.write_all(format!("\n{:<30} | {:<35.1}", c.name, allowed_debt).as_bytes()).expect("Could not write!");
            }

            if c.liabilities < 10_000_000.0 {
                let new_liability = 0.05 * c.p_leverage();
                file3.write_all(format!("\n{:<30} | {:<35.2}", c.name, new_liability).as_bytes()).expect("Could not write!");
            }
        }
        println!("\ncompany_info file updated!");
        println!("shares file updated!");
        println!("liabilities file updated!");
    } else {
        println!("You do not have access!");
    }
}