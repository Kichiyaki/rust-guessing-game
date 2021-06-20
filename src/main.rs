use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome!\nPlease input your guess.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", input);

    let secret_number = rand::thread_rng().gen_range(1..10);
    match input.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
