use std::io;

fn main() {
    println!("Hello, world!");
    let mut decision = String::new();

    loop {
        println!("Enter your option: ");
        println!("1. Addition");
        println!("2. Substraction");
        println!("3. Multiplication");
        println!("4. Division");

        io::stdin()
            .read_line(&mut decision)
            .expect("Error reading line");

        let option: i8 = decision.trim().parse().expect("Error parsing number!");
        decision.clear();

        match option {
            5 => break,
            _ => println!("Your decision is: {}", option),
        }
    }
}
