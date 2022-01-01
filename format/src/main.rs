fn main() {
    println!("Hello, world!");
    // 占位符
    println!("{0} is ok,{1} is error", "a", "b");
    println!(
        "{name} is ok,age is {age},object is {object}",
        name = "heige",
        age = 30,
        object = "rust"
    );

    println!("{:b}", 2);

    // 右对齐操作长度为6
    println!("{number:>width$}", number = 1, width = 6);

    // 你可以在数字左边补 0。下面语句输出 "000001"，填充
    println!("{number:>0width$}", number = 1, width = 6);
    println!("{}", 111) // 占位符的模式打印
}
