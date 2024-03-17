use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number = {secret_number}");

    println!("Please input your guess :");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    /* shadowing - lets us reuse the guess variable name rather than forcing us to create two unique
     variables, such as guess_str and guess
     */
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => println!("You Win"),
        Ordering::Greater => println!("Too Big")
    }
}