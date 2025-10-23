extern crate rand;

use std::{io};
use std::cmp::Ordering; 
use rand::Rng;



fn main() {

    println!("Choose your Difficulty"); 
    let mut difficulty  = String::new(); 
    println!("1. Easy \n2. Medium \n3. Hard"); 

    io::stdin().read_line(&mut difficulty).expect("There is an issue reading difficulty value");

    difficulty = difficulty.trim().to_lowercase(); 

   let (min, max, difficulty )= match difficulty.as_str() {
        "1" | "easy" => (1, 5,"easy"),
        "2" | "medium" => (1, 25,"medium"),
        "3" | "hard" => (1, 50,"hard"),            
        _ => {
        panic!("Invalid input.");
    }
       };

       println!("Difficulty is set to {}", difficulty); 




    println!("You will be Guessing the number between {} and {}", min, max); 

    let secret_number = rand::thread_rng().gen_range(min,max); 


 for _ in 0..5{

    println!("Please input your guess"); 

   let mut guess = String::new(); 
    

    io::stdin().read_line(&mut guess)
        .expect("failed to read line"); 

    
    
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Kindly Input a Valid Number");
            continue
        },
    };

    let guess =  Guess::new(guess); 

    if guess.value < min || guess.value > max {
        println! ("The secret number is between {} and {}", min, max); 
        continue; 
    }

    
    println!("you guessed {}", guess.value); 

    match guess.value.cmp(&secret_number) {
        Ordering::Less => println!("Too small"), 
        Ordering::Greater => println!("Too Big"), 
        Ordering::Equal => {
            println!("You Win");
            break; 
             },
        }

        
    }

}


    pub struct Guess {
        value: u32,
    }

    impl Guess{ 
        pub fn new (value: u32) -> Guess { 
            if value < 1 || value > 100 {
                panic!("Kndly Input a secret number between 1 and 100, your input was {} ", value); 
            }

            Guess { 
                value,
            }
        }

        pub fn value (&self) -> u32 {
            self.value
        }
    }