use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!\n===== === ======");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        print!("Please input your guess : ");
        io::stdout().flush().expect("!flush");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}