fn xor_operator(val1: i16, val2: i16) -> i16 {
    val1 ^ val2
}

fn or_operator(val1: i16, val2: i16) -> i16 {
    val1 | val2
}

fn and_operator(val1: i16, val2: i16) -> i16 {
    val1 & val2
}
fn not_operator(val: i16) -> i16 {
    !val
}

fn bitwise_l_operator(val: i16, step: i16) -> i16 {
    val << step
}

fn bitwise_r_operator(val: i16, step: i16) -> i16 {
    val >> step
}

fn main() {
    let x:i16 = 100;
    let y:i16 = 55;
    println!("{} OR {} = {}", x, y, or_operator(x, y));
    println!("{} AND {} = {}", x, y, and_operator(x, y));
    println!("{} XOR {} = {}", x, y, xor_operator(x, y));
    println!("{} bit shift to left by {} steps = {}", x, 2, bitwise_l_operator(x, 2));
    println!("{} bit shift to right by {} steps = {}", y, 1, bitwise_r_operator(y, 1));
    println!("{} NOT operation result = {}", x, not_operator(x));
}
