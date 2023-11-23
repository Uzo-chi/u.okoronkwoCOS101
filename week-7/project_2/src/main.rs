use std::{format, io};

fn main() {
    let input = io::stdin();
    let mut num = String::new();

    println!("How many siblings do you have? (Not more than 20):");
    input.read_line(&mut num).expect("Not a valid string!");
    let num:usize = num.trim().parse().expect("Not a valid integer!");

    let mut sibling_info:[[String;5];20] = [["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]];

    for i in 0..num {
        sibling_info[i] = builder();
    }

    for a in 0..num {
        println!("");
        println!("Sibling {}:", a+1);
        for j in sibling_info[a].iter() {
            if j == "" {
                continue;
            } else {
                println!("{:?}", j);
            }
        }
    }
}

fn builder() -> [String;5] {
    let input = io::stdin();
    let mut occ = String::new();
    let mut relation = String::new();
    let mut name = String::new();
    let mut age = String::new();
    let mut arr1:[String;5] = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];

    println!("What is your sibling's first name?:");
    input.read_line(&mut name).expect("Not a valid string!");
    let name = name.trim();

    println!("How old is your sibling?:");
    input.read_line(&mut age).expect("Not a valid string");
    let age:usize = age.trim().parse().expect("Not a valid integer!");

    if age > 18 {
        let mut status = String::new();

        println!("Is your sibling single or married? (Enter s or m):");
        input.read_line(&mut status).expect("Not a valid string!");
        let status:char = status.trim().parse().expect("Not a valid character");

        if status == 's' {
            let relation = "Single";
            let mut job = String::new();

            println!("Is your sibling a student or worker? (Enter student or worker):");
            input.read_line(&mut job).expect("Not a valid string!");
            let job = job.trim();

            if job == "student" {
                let occ = "Student";
                let mut uni = String::new();

                println!("What university does your sibling attend?:");
                input.read_line(&mut uni).expect("Not a valid string!");
                let uni = uni.trim();
                arr1[0] = format!("Name: {}",name);
                arr1[1] = format!("Age: {}years",age);
                arr1[2] = format!("Marital Status: {}",relation);
                arr1[3] = format!("Occupation: {}",occ);
                arr1[4] = format!("School: {}",uni);
            } else if job == "worker" {
                let occ = "Worker";
                arr1[0] = format!("Name: {}",name);
                arr1[1] = format!("Age: {}years",age);
                arr1[2] = format!("Marital Status: {}",relation);
                arr1[3] = format!("Occupation: {}",occ);
            } else {
                let arr1 = ["", "", "", "", ""];
            }
        } else if status == 'm' {
            let relation = "Married";
            let mut child = String::new();
            let mut city = String::new();

            println!("How many children does your sibling have?:");
            input.read_line(&mut child).expect("Not a valid string!");
            let child:usize = child.trim().parse().expect("Not a valid integer!");

            println!("What city does your sibling live in?:");
            input.read_line(&mut city).expect("Not a valid string!");
            let city = city.trim();
            arr1[0] = format!("Name: {}",name);
            arr1[1] = format!("Age: {}years",age);
            arr1[2] = format!("Marital Status: {}",relation);
            arr1[3] = format!("City: {}",city);
            arr1[4] = format!("Number of children: {}",child);
        } else {
            let arr1 = ["", "", "", "", ""];
        }
    } else {
        let mut waec = String::new();

        println!("Has your sibling written WASSCE? (Enter true or false):");
        input.read_line(&mut waec).expect("Not a valid string!");
        let waec = waec.trim();

        if waec == "true" {
            let wasscce = "True";
            let mut school = String::new();

            println!("What secondary school did your sibling attend?:");
            input.read_line(&mut school).expect("Not a valid string!");
            let school = school.trim();

            arr1[0] = format!("Name: {}",name);
            arr1[1] = format!("Age: {}years",age);
            arr1[2] = format!("WAEC?: {}",wasscce);
            arr1[3] = format!("School: {}",school);
        } else if waec == "false" {
            let wasscce = "False";
            let mut class = String::new();

            println!("What class is your sibling in?:");
            input.read_line(&mut class).expect("Not a valid string");
            let class = class.trim();

            arr1[0] = format!("Name: {}",name);
            arr1[1] = format!("Age: {}years",age);
            arr1[2] = format!("WAEC?: {}",wasscce);
            arr1[3] = format!("Class: {}",class);
        } else {
            let arr1 = ["", "", "", "", ""];
        }
    }
    return arr1;
}