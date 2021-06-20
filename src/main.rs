use std::cmp::Ordering;
use std::io;

use rand::Rng;

const MIN_NUMBER: u32 = 1;
const MAX_NUMBER: u32 = 10;

fn main() {
    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER..MAX_NUMBER);
    println!("Welcome!");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let input: u32 = input.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
