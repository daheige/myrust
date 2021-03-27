extern crate rand; // 导入外部组件库
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("Hello, world!");
    println!("please input your guess.");
    let mut guess = String::new(); // mut修饰可变字符串

    // 读取标准输入的内容
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    // 去掉左右两边的空格字符
    // 这里不需要重新声明变量，采用了rust的变量隐藏功能，使得之前的变量guess可以使用同一个名字
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("you guessed:{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win"),
    }

    println!("secret_number is:{}", secret_number);
}
