fn main() {
    println!("Hello, world!");
    /*
    引用与借用
    */
    // & 符号就是 引用，他们允许你使用值但不获取其所有权
    // &String s--->s1底层ptr
    // 指向值 s1 的引用，但是并不拥有它
    // 当引用离开作用域时其指向的值也不会被丢弃
    // 将获取引用作为函数参数称为 借用（borrowing）
    // 借用不能修改变量的值
    let s1 = String::from("heige");
    let len = cal(&s1); //引用传递
    println!("current s is {},len is {}", s1, len);

    // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用,不能有多个可变引用
    // 可以使用大括号来创建一个新的作用域来允许拥有多个可变引用，只是不能 同时 拥有
    let mut s = String::from("hello,heige!");
    change(&mut s);
    {
        let r1 = &mut s;
        println!("r1 is {}", r1);
    }
    // let r = &mut s; // ------ mutable borrow occurs here 会报错
    println!("s is {}", s);
    // println!("r is {}", r);

    /*
    数据竞争（data race）是一种特定类型的竞争状态，它可由这三个行为造成：
     1. 两个或更多指针同时访问同一数据。
     2. 至少有一个这样的指针被用来写入数据。
     3. 不存在同步数据访问的机制。
     数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发 生，因为它甚至不会编译存在数据竞争的代码
     */

    /* 引用规则
    * 1. 在任意给定时间，只能 拥有如下中的一个：
         一个可变引用。
        任意数量的不可变引用。
      2. 引用必须总是有效的
    */
}

fn cal(s: &String) -> usize {
    println!("s is {}", s);
    s.len()
}

fn change(s: &mut String) {
    s.push_str("abc");
}
