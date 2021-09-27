fn main() {
    println!("Hello, world!");
    task("abc", say);
    task("heige", hello);
    let mut i = incr(0, 1);
    i = incr(i, 2);
    println!("i = {}", i); // 3
    println!("1 in [1,2,3] = {}", find(2, &[1, 2, 3]));
    println!("5 in [1,2,3] = {}", find(5, &[1, 2, 3]));
    let (x, y) = muilt_value(1);
    println!("x = {},y = {}", x, y);
    let func: IncType = inc;
    println!("3+1 = {}", func(3));
    let func = inc_num; // 3+1 = 4
    println!("3*2+1 = {}", func(3)); // 3*2+1 = 7

    println!("3*2+1 = {}", inc_num(3)); // 3*2+1 = 7 函数的所有权并没有转移func

    // fn trait
    println!("3 - 1 = {}", process(3, dec));
    println!("3 + 1 = {}", process(3, inc));

    let f = get_inc_dec(1);// 结果是一个函数inc
    println!("3+1 = {}", f(3)); // 3+1 = 4
}

fn inc_num(i: i32) -> i32 {
    i * 2 + 1
}

// IncType 函数类型,type定义函数别名为IncType
// 函数类型是引用类型，它是安全的类型
type IncType = fn(i32) -> i32;

fn say(name: &str) {
    println!("name is: {}", name)
}

fn hello(name: &str) {
    println!("hell func call,name is {}", name)
}

// 将函数作为参数传递给另外一个函数
fn task(name: &str, func: fn(&str)) {
    func(name)
}

fn incr(n: i32, step: i32) -> i32 {
    n + step
}

// s是一个切片，在切片中查找指定元素
fn find(n: i32, s: &[i32]) -> bool {
    for i in s { // i是&i32类型，迭代器
        if *i == n { // 这里是取值操作，*i表示解引用操作
            return true;
        }
    }

    false
}

// fn trait 泛型函数实现，将函数作为参数传递给另外一个函数，然后执行
// process是一个高阶泛型函数
fn process<F>(i: i32, func: F) -> i32
    where F: Fn(i32) -> i32 { // where是F trait约束
    func(i)
}

fn dec(i: i32) -> i32 {
    i - 1
}

fn inc(i: i32) -> i32 {
    i + 1
}

// 函数作为返回值
fn get_inc_dec(n: i32) -> fn(i32) -> i32 {
    if n == 1 {
        inc
    } else {
        dec
    }
}

// 返回元祖模式，支持多个返回值
fn muilt_value(i: i32) -> (i32, i32) {
    (i + 1, i * 2 + 1)
}