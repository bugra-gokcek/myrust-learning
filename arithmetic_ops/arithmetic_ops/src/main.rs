fn add(num1: i8, num2: i8) -> i8 {
    num1 + num2
}

fn subside(num1: i8, num2: i8) -> i8 {
    num1 - num2
}

fn multiply(num1: i8, num2: i8) -> i8 {
    num1 * num2
}

fn divide(num1: i8, num2: i8) -> f32 {
    let res: f32 = num1 as f32 / num2 as f32;
    res
}

fn module(num1: i8, num2: i8) -> i8 {
    num1 % num2
}

fn square(num1: i8) -> i8 {
    i8::pow(num1, 2)
}
fn main() {
    let mut a: i8 = 9; //defined "a" mutable, so it can be changed in program dynamically
    a += 1; //Rust doesn't have ++ and -- operators
    let b: i8 = std::f32::consts::PI as i8; // PI converted to i8, so b is 3
    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subside(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
    println!("{} / {} = {}", a, b, divide(a, b));
    println!("{} % {} = {}", a, b, module(a, b));
    println!("Square of {} = {}", a, square(a));
}
