fn main() {
    println!("Hello, world!");
    // 闭包定义 参数放在管道||里面 {}是一个表达式
    // 闭包的 || {} 语法
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut res: i32 = x;
        res += 1;
        res += 1;
        res
    };
    assert_eq!(4, plus_two(2));
    let f = factory();
    let anwer = f(1);
    assert_eq!(6, anwer);
    assert_eq!(6, f(1));
}

//  函数调用后闭包获得了被释放的内存环境
// 通过 Box 装箱，我们提供了一个已知大小的返回值，并允许它离开我们 的栈帧
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}
