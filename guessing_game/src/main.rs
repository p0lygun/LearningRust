use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome To Guessing Game!!!");
    println!("Guess the Number ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret_number is {secret_number}");
    let mut guess = String::new();
    // mut is to make this var mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed To Read line");

    let guess: u32 = guess.trim().parse().expect("Please Type a number");

    println!("You gussed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => print!("Too Small"),
        Ordering::Greater => print!("Too Large"),
        Ordering::Equal => print!("You Win!!!"),
    }
}
