use rand::Rng;
use std::cmp::Ordering;
use std::io;

use colored::*;

pub fn guessing_game_2() {
    println!("Guess the game 2");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("input you guess, fatafati");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Whoops!, failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small it seems".red()),
            Ordering::Greater => println!("{}","Way to big, hold up big guy!".red()),
            Ordering::Equal => {
                println!("{}", "You got it right, finally".green());
                break;
            }
        }
    }

}
