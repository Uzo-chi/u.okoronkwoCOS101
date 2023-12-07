use std::io::Write;

fn main() {
    let lager: [&str;6] = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout: [&str;3] = ["Legend", "Turbo King", "Williams"];
    let n_a: [&str;4] = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = std::fs::File::create("brew.txt").expect("Could not create!");
    file.write_all("LAGER:\n".as_bytes()).expect("Could not write!");
    for item in lager.iter() {
        file.write_all(item.as_bytes()).expect("Could not write!");
        file.write_all("\n".as_bytes()).expect("Could not write!");
    }

    file.write_all("\nSTOUT:\n".as_bytes()).expect("Could not write!");
    for item in stout.iter() {
        file.write_all(item.as_bytes()).expect("Could not write!");
        file.write_all("\n".as_bytes()).expect("Could not write!");
    }

    file.write_all("\nNON-ALCOHOLIC:\n".as_bytes()).expect("Could not write!");
    for item in n_a.iter() {
        file.write_all(item.as_bytes()).expect("Could not write!");
        file.write_all("\n".as_bytes()).expect("Could not write!");
    }

    println!("Data written!");
}