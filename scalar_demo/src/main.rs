// rust中原生类型：布尔类型，char,数字类型，数组，切片，str，元祖，函数
fn main() {
    let i: i8 = 12; // 8位有符号的数字
    let m: u8 = 12; // 8位无符号的数字
    let x: i32 = 123; // 32位有符号的数字
    let y: u32 = 19; // 32位无符号的数字
    let n = 13; // 默认类型推导为i32类型
    let z: i64 = 23; // 64位的有符号的数字
    let p: isize = 12; // isize和usize依赖运行程序的计算机架构，64位架构就是64位，32位架构就是32位
    let q: u64 = 1234;
    println!("m = {}", m);
    println!("i = {}", i);
    println!("Hello, world!");
    println!("x = {},y={},n={}", x, y, n);
    println!("z = {},p={},q={}", z, p, q);

    // 浮点数
    let f: f32 = 12.3;
    let a: f64 = 123.0;
    println!("float f = {},double a = {}", f, a);

    // boolean
    let b = false;
    println!("b = {}", b);

    // 复合类型
    // 元祖
    let tup: (i32, f64, u8) = (300, 0.3, 1);

    // let变量结构
    let (x, y, z) = tup;

    println!("x = {},y={},z={}", x, y, z);
    println!("tup.0 = {}", tup.0); // 元祖按照name.0,name.1进行访问

    // 数组
    let arr = [1, 2, 3, 4, 5];

    println!("arr[1] = {}", arr[1]);
    // println!("arr[6] = {}", arr[5]); // ^^^^^^ index out of bounds: the len is 5 but the index is 5

    // 字符串
    // rust中字符串正是可变大小的数据结构，主要的字符串类型有： &str和String

    // &str叫做字符串片段，字符串常量是&'static str类型，它存储在编译好的程序中，并且整个程序的运行中一直存在
    // 任何接受一个字符串切片的函数也接受一个字符串常量
    // 把 String 转换为 &str 的代价很小，不过从 &str 转换到 String 涉 及到分配内存。除非必要，没有理由这样做！
    let greeting = "hello wolrd"; // 这是一个字符串常量，类型是&'static str ，字符串常量是静态分配的字符串切片
    println!("greeting:{}", greeting);

    // Rust的 str 类型是最原始的字符串类型
    // String是一个存在堆上分配的字符串，这个字符串可以增长，同时也是utf-8编码，String通常通过一个字符串片段调用to_string方法转化而来
    // 当然String可以通过一个&强制转化位&str,字符串片段
    let mut s = "heige".to_string();
    println!("s = {}", s);

    s.push_str(",hello");
    println!("s = {}", s);
    tasks_slice(&s);

    // 字符串遍历
    // 字节遍历
    // 字符串中每个UTF-8编 码的字符可以是多个字节，你必须遍历字符串来找到字符串的第N个字 符。这个操作的代价相当高
    for b in s.as_bytes() {
        println!("{},", b);
    }

    // 在rust中一个字符占据4个字节
    for c in s.chars() {
        println!("{},", c);
    }

    another_func(123, 6); // the value of x is 123

    let x = plus_one(123);
    println!("{}", x);
}

fn tasks_slice(s: &str) {
    println!("got:{}", s);
}

fn another_func(x: i32, y: i32) {
    println!("the value of x is {},y is {}", x, y);
}

// 函数的返回值->type 这样的风格组成
fn plus_one(x: i32) -> i32 {
    x + 10
}
