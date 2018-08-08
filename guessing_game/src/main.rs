#![feature(range_contains)]
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    use prime::*;

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

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse::<u32>() {
            Ok(num) => match (1..101).contains(&num) {
                true => Guess::new(num), // we have to do the check other wise Guess panics if created with value too big or small
                false => {
                    println!("Please type a number within range 1 to 100!");
                    continue;
                }
            },
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // println!("Size of the input is: {}", size);
        println!("You guessed: {}", guess.value());
    }
}

mod prime {
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
                // a hard example for making Guess never possible to be a value less than 1 or above 100
            }
            return Guess { value };
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}
