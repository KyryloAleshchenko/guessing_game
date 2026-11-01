use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::random_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    let mut guess_count = 0;

    loop {
        println!("Intput your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = guess.trim()
            .parse()
            .expect("Please type a valid number!");
        println!("You guessed: {}", guess);

        guess_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                println!("It took you {} attempts to guess.", guess_count);
                break;
            }

        }
    }
}   
