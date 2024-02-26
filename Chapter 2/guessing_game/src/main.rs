use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!\n===== === ======");

    print!("Please input your guess : ");
    io::stdout().flush().expect("!flush");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed {guess}");
}