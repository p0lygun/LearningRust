fn main() {
    println!("Hello, world!");
    another_function(5);
    funtion_that_is_a_statement(10);
    let x = funtion_that_is_a_expression(15);
    println!("{x}");
}
fn another_function(x: i32){
    println!("passed {x}");
}
fn funtion_that_is_a_statement(x: i32) {
    let x = x + 1;
    println!("Statement {x}");
    x;
}

fn funtion_that_is_a_expression(x: i32) -> i32{
    let x = x + 1;
    println!("exp {x}");
    x
}

// notice that in function_that_is_a_statement 
// has ; at the end of the last statement

