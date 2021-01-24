use rand::Rng;
use std::io;
fn main() {
    println!("Hello, world!");
    println!("Hello, daheige1234!");
    let mut guess = String::new(); // 这是一个可变的变量，rust默认不可变化，加上mut就可以变化
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("your guessed: {}", guess);
    let foo = 12.34;
    let mut bar = 123;
    bar = bar + 1; // 发生了变化
    println!("foo = {},bar = {}", foo, bar);

    let rnd = rand::thread_rng().gen_range(1, 101);
    println!("the rand num= {}", rnd);
    println!("fefefe");
    let a = 12;
    println!("a = {}", a);

    let b = String::from("dddd");
    println!("b = {}", b);
}
