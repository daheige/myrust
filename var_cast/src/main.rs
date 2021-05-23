use std::convert::From;
fn main() {
    println!("Hello, world!");

    // 类型显式转化
    let d = 65.4321_f32;
    let num = d as u8;
    let c = num as char;
    println!("num = {}", num); // num = 65
    println!("char = {}", c); // char = A

    let parsed: i32 = "1234".parse().expect("string to num fail");
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("sum is = {}", sum);

    let s = "hello"; // 字面量字符串
                     // 转化为String
    let my_string = String::from(s);
    println!("my_string is {}", my_string);

    let num = Number::from(30);
    println!("my number is {:?}", num);

    // i32---> Number类型
    let i = 5;
    let num: Number = i.into();
    println!("my num is {:?}", num);
    /*
    my number is Number { value: 30 }
    my num is Number { value: 5 }
    */
}

// From和Into 两个trait是内在联系，一个A->B,B->A
// 自定义类型转化机制
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
