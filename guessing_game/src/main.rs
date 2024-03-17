use std::io ;

fn main() {
    println!("Guess the number!") ;

    println!("Please input your guess :") ;

    let apple = 5 ; // immutable
    let mut banana = 12 ; // mutable


    let mut guess = String::new() ;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line") ;

    println!("You guessed: {guess}") ;

    let total = apple + banana ;
    println!("Total = {total}")
}