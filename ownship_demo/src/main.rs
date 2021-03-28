fn main() {
    println!("Hello, world!");
    // 原生类型的copy机制，仍然可用
    // 只在栈上的数据：拷贝
    // 一些copy的类型：整数，布尔类型，浮点数，元祖仅仅当其包含的类型都是copy的时候，比如(i32,i32)，对于(i32,String)类型就不是的
    let x = 1;
    let y = x;
    println!("y = {},x = {}", y, x);

    // 将x移动到m上之后，x就不能使用了
    // Rust 不需要在 x 离开作用域后清理任何东西
    let x = String::from("heige");
    let m = x;
    println!("m = {}", m);
    // println!("x = {},m = {}", x, m); // ^ value borrowed here after move

    // 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数
    let x = String::from("heige313");
    let y = x.clone();
    // 这里堆上的数据 确实 被复制了
    // 当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。你很容易 察觉到一些不寻常的事情正在发生
    println!("x = {},y = {}", x, y);

    let (s, len) = calculate_len(String::from("heige,hello"));

    println!("s = {} len = {}", s, len); // s = heige,hello len = 11

    //   & 符号就是 引用，他们允许你使用值但不获取其所有权
    let s = String::from("hello");
    println!("s len = {}", cal_len(&s)); // s = heige,hello len = 11

    let s = String::from("hello,heige");
    println!("s len = {}", cal_len2(&s));

    // 可变引用
    let mut s = String::from("hello");

    change_value(&mut s); // 传递的是一个可变引用，可以改变变量的值

    // 在特定作用域中的特定数据有且只有一个可变引用
    let mut s = String::from("hello,heige");
    // let r2 = &s;
    // let r1 = &mut s;
    // let r2 = &mut s; // 只能有一个可变引用 ^^^^^^ second mutable borrow occurs here
    // 可变引用和不可变引用不能同时存在
    // println!("r1 = {},r2 = {}", r1, r2);

    println!("s = {}", s);

    let s1 = no_dangle();

    println!("s1 = {}", s1); // s1 = hello,world

    // s1 = hello,world
    // word = 5
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // This empties the String, making it equal to "".

    println!("word = {}", word);

    // 字符串 slice
    let s = String::from("hello,heige");
    let hello = &s[0..5];
    println!("hello = {}", hello);

    let name = &s[6..];
    println!("name = {}", name);

    let s1 = &s[..];
    println!("s1 = {}", s1);

    // 字符串字面值就是 slice 具有长度，容量，指向字符串底层的指针
    let s = "heige world";
    println!("first word: {}", get_first_word(s));

    let s = String::from("hello world"); // String---> str只需要一个&s就可以了
    println!("first word: {}", get_first_word(&s));

    let mut s = String::from("hello world");
    let word = get_first_word(&s);
    // 下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。
    // 因为 clear 需要 清空 String ，它尝试获取一个可变引用，它失败了。
    // s.clear(); // ^^^^^^^^^ mutable borrow occurs here

    println!("word = {}", word);
}

fn change_value(s: &mut String) {
    s.push_str("world")
}
fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 变量 s 有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢弃它指向的数 据，
// 因为我们没有所有权。函数使用引用而不是实际值作为参数意味着无需返回值来交还所有权，因 为就不曾拥有所有权。
// 将获取引用作为函数参数称为 借用（borrowing）
fn cal_len(s: &String) -> usize {
    s.len()
}

fn cal_len2(s: &str) -> usize {
    s.len()
}

/*
 * 在存在指针的语言中，容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针 （dangling pointer），
 * 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，
 * 在 Rust 中编译器确保引用永远也不会变成悬垂状态：当我们拥有一些数据的引用，编译器确保数据 不会在其引用之前离开作用域。
 * 让我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免：
 */
// fn dangle() -> &String {
//     let s = String::from("hello");
// Here, s goes out of scope, and is dropped. Its memory goes away.
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello,world");
    s
}

/*
1.在任意给定时间，只能 拥有如下中的一个：
     a.一个可变引用。
     b.任意数量的不可变引用。
2.引用必须总是有效的
*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
