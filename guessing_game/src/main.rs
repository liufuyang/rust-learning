extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    let x = vec![1; 50];
    let mut a = 3;
    let b = a;
    let a: u32 = 4;
    println!("{}", b);
    println!("{:?}", x);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // guess address is 01, value is 123

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
            
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }

        // println!("Size of the input is: {}", size);
        println!("You guessed: {}", guess);
    }
}
