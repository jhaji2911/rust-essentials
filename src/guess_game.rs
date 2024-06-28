use rand::Rng;
use std::cmp::Ordering;
use std::io;

use colored::*;


pub fn guessing_game() {
    println!("Guess the number homie");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String :: new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //  new way to do this 
        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","To small, that's not what she said 😅".red()),
            Ordering::Greater => println!("{}","To big, that's what she said 🤣".blue()),
            Ordering::Equal => {
                println!("{}","You got it right! 🎉".green());
                break;
            }
        }
    }
}