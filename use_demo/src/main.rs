#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}

fn main() {
    println!("Hello, world!");
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;
    match status {
        Rich => println!("has lots of money"),
        Poor => println!("it is poor"),
    }

    match work {
        Civilian => println!("{}", "c work"),
        Soldier => println!("{}", "soldiers fight"),
    }

    println!("roses are {:?}", Color::Red);
    println!("roses are {:06x}", Color::Green as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list len = {}", list.len());
    println!("list string: {}", list.stringify());
}

// enum c语言风格
#[derive(Debug)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil, // 末尾节点
}

use List::*;

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, ele: u32) -> List {
        Cons(ele, Box::new(self))
    }

    fn len(&self) -> u32 {
        //  `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        //  类型要好过匹配引用 `&T`
        match *self {
            //  不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => format!("nil"),
        }
    }
}
