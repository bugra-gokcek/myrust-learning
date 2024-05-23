fn func()
{
    let a: u8 = 255;
    println!("value of a in func scope : {}", a);
}
fn main() {
    let a: u8 = 0;
    println!("value of a in main scope : {}", a);
    {
        let a: u8 = 127;
        println!("value of a in main-inside scope : {}", a);
    }
    func();
}
