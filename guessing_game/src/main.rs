use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome To Guessing Game!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the Number ");

        let mut guess = String::new();
        // mut is to make this var mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue 
            // underscore is a catchall keyword, in this case we dont care for what the ERROR object has in it
        };

        println!("You gussed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
        }
    }
}
