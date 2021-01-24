// 编写一个宏,要先定义，然后才可以使用
// 元变量 $x 被解析成一个单独的表达式节点，并且在替换后依旧在语法 树中保持原值
macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

fn main() {
    println!("Hello, world!");
    assert_eq!(25, five_times!(2 + 3));
}
