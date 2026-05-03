use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    println!("Guess the Number");
    println!("Please enter your guess.");

    let master_number = rand::thread_rng().gen_range(1, 501);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read ");
        println!("You guessed = {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a number");
                continue;
            }
        };
        match guess.cmp(&master_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal =>{
                 println!("{}","You win".green());
                 break;
            }
        }

    }
}
