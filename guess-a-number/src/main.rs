use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Enter a number of tries:");
    let mut num_of_tries = String::new();
    io::stdin()
        .read_line(&mut num_of_tries)
        .expect("Failed to read a line");
    let mut num_of_tries: u32 = num_of_tries.trim().parse().expect("Expected unsigned int");
    println!("You set {} number of tries", num_of_tries);
    println!("Guess a number btw 0 and 100");

    let secret = rand::thread_rng().gen_range(0..=100);
    let mut guessed: bool = false;

    for i in 0 .. num_of_tries {
        let mut guess = String::new();
        println!("Please enter your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let mut guess: u32 = guess.trim().parse().expect("Expected unsigned integer");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Think greater"),
            Ordering::Greater => println!("Think less"),
            Ordering::Equal => {
                guessed = true;
                println!("You are god damn right!");
                break;
            }
        }
    }

    if (guessed) {
        println!("Congrats!!!!");
    } else {
        println!("Ooops, number of tries is reached :(");
    }

}
