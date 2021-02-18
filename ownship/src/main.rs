fn main() {
    println!("Hello, world!");
    let x: i32 = 12;
    println!("x = {}", x);

    {
        let y: i32 = 1;
        println!("y = {}", y);
    }

    // ^ help: a local variable with a similar name exists: `x`
    // println!("y = {}", y);

    // 块作用域
    {
        // 类型大小不固定，分配在堆上
        let mut s1 = String::from("heige");
        s1.push_str(" hello");
        println!("s1 = {}", s1);
        println!("len = {}", s1.len());
        println!("cap = {}", s1.capacity());

        // s1 move to s2
        let s2 = s1;

        println!("s2 = {}", s2);
        // println!("s1 = {}", s1); // ^ value borrowed here after move

        // clone
        let s3 = s2.clone(); // 深拷贝
        println!("s2 = {} s3 = {}", s2, s3);
    }

    // copy 栈上的拷贝，依然可以继续使用
    let a = 1;
    let b = a;
    println!("a = {} b = {}", a, b);

    // 所有权
    // 常用的具有copy trait特征的，复制之后依然可以继续使用
    // 所有的整型
    // 浮点型
    // boolean类型
    // char字符类型
    // tuple元组

    let s = String::from("hello");
    // 进入一个新的作用域，当离开作用域后s就不能继续使用
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    // println!("s = {},x = {}", s, x);
    // ^ value borrowed here after move
    println!("x = {}", x);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}
