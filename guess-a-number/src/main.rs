use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number btw 0 and 100");

    let secret = rand::thread_rng().gen_range(0..=100);

    println!("Please enter your guess:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let mut guess: u32 = guess.trim().parse().expect("Expected unsigned integer");
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Think greater"),
            std::cmp::Ordering::Greater => println!("Think less"),
            std::cmp::Ordering::Equal => {
                println!("You are god damn right!")
            }
        }
    }

}
