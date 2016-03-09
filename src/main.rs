extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess away!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Input your guess. Pssst, it should be {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {}", guess);

    let guess: u32 = guess.trim().parse().expect("Has to be a number, buddy!");

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Not quite"),
        Ordering::Greater => println!("Whoah there, cowboy!"),
        Ordering::Equal   => println!("Spot on!"),
    }
}
