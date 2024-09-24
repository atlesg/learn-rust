// Input and Output library
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    // let: create a variable
    // mut: make the variable mutable
    // String::new(): create a new empty string
    // &mut guess: a mutable reference to the string
    let mut guess = String::new();

    // read_line(): read a line from the standard input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
