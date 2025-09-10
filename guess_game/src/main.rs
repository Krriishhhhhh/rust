use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Wellcome to guess Game :");

    loop {
        println!("Make your Guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Enter a number Only"),
        };

        let secret_number = rand::thread_rng().gen_range(1..=100);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess was short "),
            Ordering::Equal => {
                println!("Bingoo");
                break;
            }
            Ordering::Greater => println!("Guess was ahead"),
        }
    }
}
