fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new(); // 创建一个空的vector,可以存放多个slice元素
    v.push(12);
    v.push(123);
    println!("{:?}", v);
    println!("{}", v[0]);
    println!("{}", v[1]);

    let mut v2 = vec![1, 3, 4, 9]; // 自动推导类型
    v2.push(12); // 添加一个元素，这里v2必须是可变类型的slice
    println!("{}", v2[0]);

    let two = &v2[1];
    println!("{}", two);
    // let third = v2.get(3);
    for i in &v2 {
        println!("value = {}", i);
    }

    println!("========change value=====");
    // 改变v2元素的值，这里需要通过可变引用访问v2
    for val in &mut v2 {
        println!("value = {}", val);
        *val += 1;
    }

    println!("=============");
    for i in &v2 {
        println!("value = {}", i);
    }

    let v3 = vec![
        Spred::Int(1),
        Spred::Float(9.08),
        Spred::Text(String::from("hello")),
    ];

    for val in &v3 {
        println!("{:?}", val);
    }

    let s = "abc黑哥"; // 字符串占据的字节的长度，这里每一 个字母的 UTF-8 编码都占用一个字节
    println!("{}", s.len()); // 9

    for c in s.chars() {
        println!("{}", c);
    }
}

// 通过枚举实现vector slice里面可以存放多个不同的元素
#[derive(Debug)]
enum Spred {
    Int(i32),
    Float(f64),
    Text(String),
}
