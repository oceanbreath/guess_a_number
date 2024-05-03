use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("----------------------------------");
    println!("---     Let's play a game!     ---");
    println!("--- You have to guess a number ---");
    println!("---  from 1 to 100 inclusive.  ---");
    println!("----------------------------------");
    let guessed_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Number '{}' is ", guess);

        match guess.cmp(&guessed_number) {
            Ordering::Less => print!("too small. "),
            Ordering::Greater => print!("too big. "),
            Ordering::Equal => {
                println!("the winning number!");
                println!("-----------------------------------");
                println!("You win! Congratulations!");
                break;
            }
        }
    }
}
