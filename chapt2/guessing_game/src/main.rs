extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0,101);
    loop{
           println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue
    };


    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {println!("Exactly right!"); break;},
        Ordering::Greater => println!("Too big!")
    } 
    }

}
