use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    let num = 123;
    let secret_num = 1234;
    match num.cmp(&secret_num) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win"),
    }
}
