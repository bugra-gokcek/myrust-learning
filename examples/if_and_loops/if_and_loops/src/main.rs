fn odd_or_even(num: i16) -> &'static str {
    if num % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

fn less_or_more(num: i16) -> &'static str {
    if num > 0 {
        "Positive"
    } else if num < 0 {
        "Negative"
    } else {
        "Zero"
    }
}

fn while_loop() {
    //a while loop which is printing even numbers from 1 to 10
    let mut i: i16 = 0;
    let mut state: &str;
    while i < 10 {
        if i == 0 {
            i += 1;
            continue; //pass the zero
        }
        state = odd_or_even(i); //get i odd or even
        if state == "Even" {
            println!("{}", i)
        }
        i += 1;
    }
}

fn vanilla_loop() {
    let mut i = 1;
    let mut counter: i8 = 0;
    loop {
        println!("Step {}, value of i is {}", counter, i);
        i *= 2;
        counter += 1;
        if counter > 10 {
            break;
        }
    }
}

fn for_loop() {
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("sum is {}", sum);
    for (_index, x) in (100..106).enumerate() {
        //enumerate method for counting iterations
        println!("value of {}. index = {}", _index, x);
    }
}
fn main() {
    //if examples
    println!("{}", odd_or_even(5));
    println!("{}", odd_or_even(6));
    println!("{}", less_or_more(18));
    println!("{}", less_or_more(-37));
    println!("{}", less_or_more(0));
    //while loop
    while_loop();
    //loop
    vanilla_loop();
    //for loop
    for_loop();
}
