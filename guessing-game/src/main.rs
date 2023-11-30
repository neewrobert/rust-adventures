use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess a number");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut counter = 0;
    loop {
        guess.clear();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed {} times", counter);
                break;
            }
        }

        counter += 1;
    }
}
