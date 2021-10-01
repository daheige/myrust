fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("v4 is:{:?},v6 is: {:?}", four, six);
    let ip = IpAdrr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("ip is:{}", ip.address);

    //let ip2 = IpAdrr2::V4(String::from("127.0.0.1"));
    //println!("ip2 is:{:?}", ip2);

    // Option特殊枚举
    let c = Coin::Penny;
    println!("c u32 = {}", value_in_cents(c));

    let five = Some(5);
    let six = plus_one(five);
    println!("six :{:#?}", six);

    let x = 11;
    match x {
        1 => println!("1"),
        3 => println!("3"),
        _ => (), // _通配符，不满足上面的所有条件，什么都不做
    }

    let u8 = Some(3);
    match u8 {
        Some(3) => println!("three"),
        _ => (),
    }

    // 和下面的if..let写法一样
    // if let 获取通过 = 分隔的一个模式和一个表达式
    // 可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略 所有其他值
    let u8 = Some(1);
    if let Some(3) = u8 {
        println!("three");
    } else {
        println!("no three");
    }
}

/*
 * Option<T> 类型是如何帮助你利用类型系统来避免出错的。要么包含值，要么没有
 <T> 它是一个泛型类型参数
 当枚举值包含数据时，你可以根据需要 处理多少情况来选择使用 match 或 if let 来获取并使用这些值
 match允许我们将一个值与一系列的模式相比较，并根据相匹 配的模式执行相应代码。
 模式可由字面值、变量、通配符和许多其他内容构成
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            // => 将模式和运行的代码分开
            println!("luck coin");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
struct IpAdrr {
    kind: IpAddrKind,
    address: String, // 地址
}

// 仅仅使用枚举并将数据直接放进每一个枚举成员 而不是将枚举作为结构体的一部分
// enum IpAdrr2 {
//     V4(String),
//     V6(String),
// }

#[derive(Debug)]
enum IpAddrKind {
    V4,
    // 首字母必须大写
    V6,
}
