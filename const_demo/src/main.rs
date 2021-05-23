// 使用static关键字声明声明全局变量，包含了要求的生命周期 ‘static
// const用来声明常量，不可更改
static LANG: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("Hello, world!");
    let n = 12;
    println!("this threshold is {}", is_big(n));
    println!("lang is {}", LANG);
}
