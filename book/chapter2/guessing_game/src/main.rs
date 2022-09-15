use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    
    loop {

        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number next time!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small guess!"),
            Ordering::Greater => println!("Too big guess!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
