use std::io;

fn main() {
    let input = io::stdin();
    let mut choice = String::new();

    println!("Area of trapezium (Type t)\nArea of rhombus (Type r)");
    println!("Area of parallelogram (Type p)\nArea of cube (Type c)");
    println!("Volume of cylinder (Type v)");
    println!("\nWhich formula would you like to use:");

    input.read_line(&mut choice).expect("Not a valid string!");
    let choice:char = choice.trim().parse().expect("Not a valid character!");

    if choice == 't' {
        let mut height = String::new();
        let mut base1 = String::new();
        let mut base2 = String::new();

        println!("Enter the height of the trapezium (in decimal):");
        input.read_line(&mut height).expect("Not a valid string!");
        let height:f64 = height.trim().parse().expect("Not a valid float!");

        println!("Enter the length of the first base (in decimal):");
        input.read_line(&mut base1).expect("Not a valid string!");
        let base1:f64 = base1.trim().parse().expect("Not a valid float!");

        println!("Enter the length of the second base (in decimal):");
        input.read_line(&mut base2).expect("Not a valid string!");
        let base2:f64 = base2.trim().parse().expect("Not a valid float!");

        let a = a_trapezium(height, base1, base2);

        println!("The area of the trapezium is {}", a);
    } else if choice == 'r' {
        let mut diag1 = String::new();
        let mut diag2 = String::new();

        println!("Enter the length of the first diagonal (in decimal):");
        input.read_line(&mut diag1).expect("Not a valid string!");
        let diag1:f64 = diag1.trim().parse().expect("Not a valid float!");

        println!("Enter the length of the second diagonal (in decimal):");
        input.read_line(&mut diag2).expect("Not a valid string!");
        let diag2:f64 = diag2.trim().parse().expect("Not a valid float!");

        let a = a_rhombus(diag1, diag2);

        println!("The area of the rhombus is {}", a);
    } else if choice == 'p' {
        let mut base = String::new();
        let mut alt = String::new();

        println!("Enter the length of the base (in decimal):");
        input.read_line(&mut base).expect("Not a valid string!");
        let base:f64 = base.trim().parse().expect("Not a valid float!");

        println!("Enter the height (in decimal):");
        input.read_line(&mut alt).expect("Not a valid string!");
        let alt:f64 = alt.trim().parse().expect("Not a valid float!");

        let a = a_parallelogram(base, alt);

        println!("The area of the parallelogram is {}", a);
    } else if choice == 'c' {
        let mut length = String::new();

        println!("Enter the length of the side (in decimal):");
        input.read_line(&mut length).expect("Not a valid string!");
        let length:f64 = length.trim().parse().expect("Not a valid float!");

        let a = a_cube(length);

        println!("The area of the cube is {}", a);
    } else if choice == 'v' {
        let mut radius = String::new();
        let mut height = String::new();

        println!("Enter the length of the radius (in decimal):");
        input.read_line(&mut radius).expect("Not a valid string!");
        let radius:f64 = radius.trim().parse().expect("Not a valid float!");

        println!("Enter the height of the cylinder (in decimal):");
        input.read_line(&mut height).expect("Not a valid string!");
        let height:f64 = height.trim().parse().expect("Not a valid float!");

        let a = v_cylinder(radius, height);

        println!("The volume of the cylinder is {}", a);
    } else {
        println!("There is no option available for that.");
    }
}

fn a_trapezium(h:f64, b1:f64, b2:f64) -> f64 {
    let area = (h / 2.0) * (b1 + b2);
    return area;
}

fn a_rhombus(d1:f64, d2:f64) -> f64 {
    let area = (d1 * d2) / 2.0;
    return area;
}

fn a_parallelogram(b:f64, h:f64) -> f64 {
    let area = b * h;
    return area;
}

fn a_cube(l:f64) -> f64 {
    let area = 6.0 * l * l;
    return area;
}

fn v_cylinder(r:f64, h:f64) -> f64 {
    let area = (22.0/7.0) * r * r * h;
    return area;
}