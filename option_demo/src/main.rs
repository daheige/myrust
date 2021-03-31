fn main() {
    println!("Hello, world!");
    // Option 枚举和其相对于空值的优势
    // Option 类型应用广泛因为它编码了一个非常普遍的场景，
    // 即一个值要么是某个值要么什么都 不是。从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，
    // 这样就可以避免在其他编程语言中非常常见的 bug。
    // Rust 并没有空值，不过它确实拥有一个可以编码 存在或不存在概念的枚举。这个枚举是 Option<T>

    let x = value_in_cents(Coin::Penny);

    println!("x = {}", x);
    println!("{}", value_in_cents(Coin::Quarter));

    let five = Some(5);
    let six = plus_one(five);

    println!("{:?}", six); // Some(6)
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}
