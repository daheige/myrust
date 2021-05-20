fn main() {
    let age: Result<i8, _> = "34".parse(); // 返回Result<T,E>
    if let Ok(a) = age {
        // if let Ok
        println!("age is {}", a);
    };

    // Option类型 Option<T> 表示是否有值，如果有值的话存放在Some(value),没有值的话就是None
    let favorite_color: Option<&str> = Some("red");
    if let Some(color) = favorite_color {
        // if let Some
        println!("color is {}", color);
    }

    // while let Some
    let mut v = vec![1, 2, 3, 4];
    v.push(10);
    v.push(6);

    while let Some(val) = v.pop() {
        // 从末尾弹出一个元素
        // pop返回的是一个
        println!("value = {}", val);
    }

    // for循环迭代
    let v = vec!["a", "b", "c"];
    for (key, val) in v.iter().enumerate() {
        println!("key = {},value = {}", key, val);
    }

    // let 变量解构
    let (x, y, z) = (1, 2.3, 4);
    println!("x = {},y = {},z = {}", x, y, z);

    // 使用 _ 模式忽略值
    let (x, y, _) = (12, 13, 15);
    println!("x = {},y = {}", x, y);

    // 传递给函数的元组拆分为值
    let point = (1, 2);
    print_color(&point);

    // match匹配字面量
    let x = 12;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        13 => println!("3"),
        _ => println!("anything"), // 都不满足的情况下执行
    }

    // 匹配命名变量
    let x = Some(50);
    let y = 10;
    match x {
        Some(5) => println!("ok"),
        Some(y) => println!("hello,y = {}", y), // x能匹配，这里的y不是上面定义的y,而是匹配到x,然后复制y=50
        // 一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了
        _ => {
            println!("default x = {:?}", x);
        }
    }

    println!("at the end x = {:?} y = {:?}", x, y);

    // match多个模式匹配,可以使用 | 语法匹配多个模式，它代表 或（or）的意思
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("3"),
        _ => println!("nothing"),
    }

    // match range范围匹配
    // 通过 ..= 匹配值的范围
    // 范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。
    // char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型
    let y = 2;
    match y {
        1..=5 => {
            println!("match range y = {}", y);
        }
        _ => println!("match nothing"),
    }

    // let解构结构体
    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;
    println!("point x = {},y = {}", x, y);

    let p = Point { x: 1, y: 2 };
    match p {
        Point { x: 0, y } => println!("match x =0,y = {} success", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y), // On neither axis: (1, 2)
    }

    // match enum
    let color = Message::ChangeColor(1, 2);
    match color {
        Message::Quit => println!("quit"),
        Message::Running => println!("running..."),
        Message::Write(x) => println!("write data...{}", x),
        Message::Move { x, y } => println!("move data...{},y = {}", x, y),
        Message::ChangeColor(x, y) => println!("change color: x = {},y = {}", x, y),
    }

    // let解构复杂类型 feet:3 inches = 10, point x = 3,y = -10
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!(
        "feet:{} inches = {}, point x = {},y = {}",
        feet, inches, x, y
    );

    // 忽略函数参数
    foo(1, 3);

    // Some(_) 忽略匹配的值

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 1; // 忽略警告
    let y = 2;
    println!("y = {}", y);

    // 用 .. 忽略剩余值
    let (x, y, ..) = (1, 2, 3, 4);
    println!("x = {},y = {}", x, y);

    // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件
    // 它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
    let x = Some(4);
    match x {
        Some(y) if y < 5 => println!("y < 5"),
        Some(y) => println!("current y = {}", y),
        None => println!("no match"),
    }

    // @ 绑定 at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
    // 使用 @ 可以在一个模式中同时测试和保存变量值
    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello { id: id @ 1..=10 } => println!("match msg id = {} success", id), // match msg id = 5 success
        Message2::Hello { id } => println!("current id = {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Running,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32),
}
// 将元组的元素展开解构传递给函数
// 匹配模式匹配 &(x, y)
fn print_color(&(x, y): &(i32, i32)) {
    println!("color x = {},y = {}", x, y);
}

// _忽略参数
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

enum Message2 {
    Hello { id: i32 },
}

/*
模式是 Rust 中一个很有用的功能，它帮助我们区分不同类型的数据。
当用于 match 语句时，Rust 确保模式会包含每一个可能的值，否则程序将不能编译。
let 语句和函数参数的模式使得这些结构更强大，可以在将值解构为更小部分的同时为变量赋值。
可以创建简单或复杂的模式来满足我们的要求
 */
