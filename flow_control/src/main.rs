use std::io;

fn main() {
    let mut choice = String::new();
    
    println!("Choose example");
    println!("1. F -> C");
    println!("2. nth fibo");

    io::stdin()
        .read_line(&mut choice)
        .expect("Unable to get choice");    
    
    let choice : i32 = choice.trim().parse().expect("Enter a number");
    let mut input = String::new();

    if choice == 1 {
        println!("Enter temp in F");
        io::stdin()
            .read_line(&mut input)
            .expect("Enable to read input");
        let input : f32 = input.trim().parse().expect("Please enter a number");
        let t_in_c: f32 = f2c(input);
        println!("{input}f is {t_in_c}c in celsius");    
    } else if choice == 2 {
        println!("Enter n");
        io::stdin()
            .read_line(&mut input)
            .expect("Enable to read input");
        let input : i32 = input.trim().parse().expect("Please enter a number");
        const GOLDEN_RATIO : f32 = 1.618034;
        println!(
            "Fib till nth is {0}", 
            ((GOLDEN_RATIO.powi(input) - (1.0 - GOLDEN_RATIO).powi(input)) / 5.0_f32.powf(0.5)) as i32
        )
                
    } else {
        println!("Enter Correct Number")
    }
        
    
}

fn f2c(t: f32) -> f32 {
    (t - 32.0) * (5.0 / 9.0)
}