fn main() {
    println!("Hello, world!");
    let num = 3;

    // if语句
    if num < 5 {
        println!("conditon was true");
    } else {
        println!("condition was false");
    }

    let c = true;
    let x = if c { 10 } else { 12 };

    println!("x = {}", x);

    // Rust 有三种循环类型： loop 、 while 和 for
    let mut i = 0;
    loop {
        if i > 10 {
            break;
        }

        println!("current i = {}", i);
        i += 1;
    }

    let mut num = 3;

    while num != 0 {
        println!("num is {}", num);
        num -= 1;
    }

    let a = [1, 3, 4, 5, 6];
    for ele in a.iter() {
        println!("the value is {}", ele);
    }

    for num in (1..10).rev() {
        println!("num = {}", num);
    }
}
