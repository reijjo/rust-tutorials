use std::io; // Input/Output standard library
use std::cmp::Ordering; // Ordering is enum with LESS, GREATER and EQUAL
use rand::Rng; // Random number generater with range

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Creates random number 1-100

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new(); // mutable string (you can change the value)
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
