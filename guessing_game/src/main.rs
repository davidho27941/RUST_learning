use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Please guess a number.");
    println!("Please input a number:");
    let secret_number = rand::thread_rng()
                                .gen_range(1,101); // Using `rand` library to generate a random number in domain [1,100]
    loop{
        let mut guess = String::new();          // Create a mutable variable `guess`. 

        io::stdin()                             // Calling standard input function. 
            .read_line(&mut guess)              // Calling read_line function to get the input value.
            .expect("Faild to load the number! Please try again!"); // A generic type to deal with exception. Return `OK` or `Err` to specify the result.

        println!("The number you guess: {}", guess);    
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  // If the transformation be execute successfully, return a number.
            Err(_) => continue,  // If the transformation failed, go to next loop.
        };
        match guess.cmp(&secret_number) {      // A match method can help to deal with the comparison. 
            Ordering::Less => println!("Too small!"),   // If guess number less than secret number, show this message.
            Ordering::Greater => println!("Too big!"),  // If guess number greater than secret number, show this message.
            Ordering::Equal => {
                println!("You win!");
                break;
            }   // If guess number equal secret number, show this message.
        }
    }
}
