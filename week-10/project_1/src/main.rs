struct Laptop {
    brand:String,
    price:usize,
    amount:usize
}

impl Laptop {
    fn cost(&self) -> usize {
        self.amount * self.price
    }
}

fn main() {
    let p1 = Laptop {
        brand:String::from("HP"),
        price:650_000,
        amount:3
    };
    let p2 = Laptop {
        brand:String::from("IBM"),
        price:755_000,
        amount:3
    };
    let p3 = Laptop {
        brand:String::from("Toshiba"),
        price:550_000,
        amount:3
    };
    let p4 = Laptop {
        brand:String::from("Dell"),
        price:850_000,
        amount:3
    };

    let total_cost = p1.cost() + p2.cost() + p3.cost() + p4.cost();

    println!("The total cost of the laptops is N{}", total_cost);
}