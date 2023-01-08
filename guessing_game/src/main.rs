use std::io;

fn main() {
    println!("Welcome To Guessing Game!!!");
    println!("Guess the Number ");

    let mut guess = String::new();  
    // mut is to make this var mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed To Read line");

    println!("You gussed {guess}")
}
