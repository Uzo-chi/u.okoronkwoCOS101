use std::io;

fn main() {
    let input = io::stdin();
    let o_admin = ["Intern", "Administrator", "Senior Administrator",
        "Office Manager", "Director", "CEO"];
    let academic = ["Research Assistant", "PhD Candidate", "Post-Doc Researcher",
        "Senior Lecturer", "Dean"];
    let lawyer = ["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2",
        "Senior Associate 3-4", "Partner"];
    let teacher = ["Placement", "Classroom Teacher", "Senior Techer", "Leading Teacher",
        "Deputy Principal", "Principal"];
    let mut staff_info: Vec<&str> = Vec::new();
    let mut occ = String::new();
    let mut position = String::new();

    print!("\nOffice Administrator\tAcademic\t\tLawyer\t\t\tTeacher\n");
    print!("-----------------------------------------------------------------------------------------\n");
    print!("Intern\t\t\t\t\t\tParalegal\t\tPlacement\n");
    print!("Administrator\t\tReasearch Assistant\tJunior Associate\tClassroom Teacher\n");
    print!("Senior Administrator\tPhD Candidate\t\tAssociate\t\tSenior Teacher\n");
    print!("Office Manager\t\tPost-Doc Researcher\tSenior Associate 1-2\tLeading Teacher\n");
    print!("Director\t\tSenior Lecturer\t\tSenior Associate 3-4\tDeputy Principal\n");
    print!("CEO\t\t\tDean\t\t\tPartner\t\t\tPrincipal\n");

    println!("\nWhat is your occupation?:\nOffice Administrator (o)\nAcademic (a)\nLawyer (l)");
    println!("Teacher (t)");
    input.read_line(&mut occ).expect("Could not read");
    let occ:char = occ.trim().parse().expect("Not a character");

    if occ == 'o' {
        staff_info.push("Office Administrator");
        let mut years = String::new();

        println!("What is your position?(From table above):");
        input.read_line(&mut position).expect("Could not read");
        let position = position.trim();

        for i in 0..o_admin.len() {
            if position == o_admin[i] {
                staff_info.push(&position);
            }
        }

        println!("How many years of work experience?:");
        input.read_line(&mut years).expect("Could not read");
        let years:i32 = years.trim().parse().expect("Not a valid integer");

        if position == "Intern" && (years > 0 && years <= 2) {
            let level = "APS 1-2";
            staff_info.push(&level);
        } else if position == "Administrator" && (years > 2 && years <= 5) {
            let level = "APS 3-5";
            staff_info.push(&level);
        } else if position == "Senior Administrator" && (years > 4 && years <= 8) {
            let level = "APS 5-8";
            staff_info.push(&level);
        } else if position == "Office Manager" && (years > 7 && years <= 10) {
            let level = "EL1 8-10";
            staff_info.push(&level);
        } else if position == "Director" && (years > 9 && years <= 13) {
            let level = "EL2 10-13";
            staff_info.push(&level);
        } else if position == "CEO" && years >= 13 {
            let level = "SES";
            staff_info.push(&level);
        } else {
            println!("Sorry, there is no specified level for you!");
        }
    } else if occ == 'a' {
        staff_info.push("Academic");
        let mut years = String::new();

        println!("What is your position?(From table above):");
        input.read_line(&mut position).expect("Could not read");
        let position = position.trim();

        for i in 0..academic.len() {
            if position == academic[i] {
                staff_info.push(&position);
            }
        }

        println!("How many years of work experience?:");
        input.read_line(&mut years).expect("Could not read");
        let years:i32 = years.trim().parse().expect("Not a valid integer");

        if position == "" && (years > 0 && years <= 2) {
            let level = "APS 1-2";
            staff_info.push(&level);
        } else if position == "Research Assistant" && (years > 2 && years <= 5) {
            let level = "APS 3-5";
            staff_info.push(&level);
        } else if position == "PhD Candidate" && (years > 4 && years <= 8) {
            let level = "APS 5-8";
            staff_info.push(&level);
        } else if position == "Post-Doc Researcher" && (years > 7 && years <= 10) {
            let level = "EL1 8-10";
            staff_info.push(&level);
        } else if position == "Senior Lecturer" && (years > 9 && years <= 13) {
            let level = "EL2 10-13";
            staff_info.push(&level);
        } else if position == "Dean" && years >= 13 {
            let level = "SES";
            staff_info.push(&level);
        } else {
            println!("Sorry, there is no specified level for you!");
        }
    } else if occ == 'l' {
        staff_info.push("Lawyer");
        let mut years = String::new();

        println!("What is your position?(From table above):");
        input.read_line(&mut position).expect("Could not read");
        let position = position.trim();

        for i in 0..lawyer.len() {
            if position == lawyer[i] {
                staff_info.push(&position);
            }
        }

        println!("How many years of work experience?:");
        input.read_line(&mut years).expect("Could not read");
        let years:i32 = years.trim().parse().expect("Not a valid integer");

        if position == "Paralegal" && (years > 0 && years <= 2) {
            let level = "APS 1-2";
            staff_info.push(&level);
        } else if position == "Junior Associate" && (years > 2 && years <= 5) {
            let level = "APS 3-5";
            staff_info.push(&level);
        } else if position == "Associate" && (years > 4 && years <= 8) {
            let level = "APS 5-8";
            staff_info.push(&level);
        } else if position == "Senior Associate 1-2" && (years > 7 && years <= 10) {
            let level = "EL1 8-10";
            staff_info.push(&level);
        } else if position == "Senior Associate 3-4" && (years > 9 && years <= 13) {
            let level = "EL2 10-13";
            staff_info.push(&level);
        } else if position == "Partner" && years >= 13 {
            let level = "SES";
            staff_info.push(&level);
        } else {
            println!("Sorry, there is no specified level for you!");
        }
    } else if occ == 't' {
        staff_info.push("Teacher");
        let mut years = String::new();

        println!("What is your position?(From table above):");
        input.read_line(&mut position).expect("Could not read");
        let position = position.trim();

        for i in 0..teacher.len() {
            if position == teacher[i] {
                staff_info.push(&position);
            }
        }

        println!("How many years of work experience?:");
        input.read_line(&mut years).expect("Could not read");
        let years:i32 = years.trim().parse().expect("Not a valid integer");

        if position == "Placement" && (years > 0 && years <= 2) {
            let level = "APS 1-2";
            staff_info.push(&level);
        } else if position == "Classroom Teacher" && (years > 2 && years <= 5) {
            let level = "APS 3-5";
            staff_info.push(&level);
        } else if position == "Senior Teacher" && (years > 4 && years <= 8) {
            let level = "APS 5-8";
            staff_info.push(&level);
        } else if position == "Leading Teacher" && (years > 7 && years <= 10) {
            let level = "EL1 8-10";
            staff_info.push(&level);
        } else if position == "Deputy Principal" && (years > 9 && years <= 13) {
            let level = "EL2 10-13";
            staff_info.push(&level);
        } else if position == "Principal" && years >= 13 {
            let level = "SES";
            staff_info.push(&level);
        } else {
            println!("Sorry, there is no specified level for you!");
        }
    }

    for index in 0..3 {
        if index == 0 {
            println!("\nOccupation: {}", staff_info[index]);
        } else if index == 1 {
            println!("Position: {}", staff_info[index]);
        } else {
            println!("Level: {}", staff_info[index]);
        }
    }
}