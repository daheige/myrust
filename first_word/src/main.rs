fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    let index = first_word(&s);
    s.clear(); // 清空字符串
    println!("index is : {}", index);

    // start..end语法
    /*
    字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内
    如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出
    */
    let s = String::from("hello world");
    println!("s1 is :{}", &s[0..5]);
    println!("s2 is :{}", &s[6..11]);
    println!("s3 is :{}", &s[..s.len()]);
    println!("s4 is {}", first_word2(&s));

    // 字符串字面值就是 slice
    // 它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符 串字面值是不可变的； &str 是一个不可变引用
    let s2 = "daheige"; // s类型是&str
    println!("s2 is {}", s2);

    // 字符串 slice 作为参数
    let s = "abc heige"; // 字面量str slice
    println!("s4 is {}", first_word2(&s[..]));

    println!("s4 is {}", first_word2(s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // iter()迭代后，调用enumerate()返回一个元祖，元祖的第一个式index,第二个式对于元素的引用
        // &item单个字节
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 定义一个获取字符串 slice 而不是字符串引用的函数使得我们的 API 更加通用并且 不会丢失任何功能
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // iter()迭代后，调用enumerate()返回一个元祖，元祖的第一个式index,第二个式对于元素的引用
        // &item单个字节
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

/*
 * 所有权、借用和 slice 这些概念是 Rust 可以在编译时保障内存安全的关键所在。
 * Rust 像其他 系统编程语言那样给予你对内存使用的控制，但拥有数据所有者在离开作用域后自动清除其数据的功 能意味着你无须额外编写和调试相关的控制代码。
 */
