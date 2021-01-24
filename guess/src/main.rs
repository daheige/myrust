extern crate rand; // 导入外部组件库
use rand::Rng;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("Hello, world!");
    println!("please input your guess.");
    let mut guess = String::new(); // mut修饰可变字符串

    // 读取标准输入的内容
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("you guessed:{}", guess);
    println!("secret_number is:{}", secret_number);
}
