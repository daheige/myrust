fn main() {
    println!("Hello, world!");
    let c = true;
    // if..else 声明变量
    let x = if c { 3 } else { 123 };
    println!("x = {}", x);

    // loop循环使用
    let mut i = 0;
    loop {
        if i > 100 {
            break;
        }

        println!("current i = {}", i);
        i += 1;
    }

    // loop break作为返回值
    let res = loop {
        i += 1;
        if i == 120 {
            break i;
        }
    };
    println!("res = {}", res);

    // while循环
    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }

    // for
    // 数组定义格式[Type;N]
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for e in arr.iter() {
        println!("ele = {}", e);
    }

    // 另一种方式实现for循环
    for e in &arr {
        println!("ele = {}", e);
    }
}
