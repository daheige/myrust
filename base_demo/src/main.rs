fn main() {
    let arr = [1, 2, 3];
    println!("arr = {}", arr[0]);
    let a = [3; 5];
    println!("arr = {}", a[0]);
    println!("arr = {:?}", a); // [3, 3, 3, 3, 3]

    let num = 3;
    if num < 5 {
        println!("num < 5");
    } else {
        println!("num >= 5");
    }

    let mut c = 0;
    // let ... loop用法
    let res = loop {
        if c >= 10 {
            break c;
        }

        println!("current c = {}", c);
        c += 1;
    };

    println!("res = {}", res); // res = 10

    let mut n = 10;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }

    // for in遍历
    for x in [1, 2, 3, 4].iter() {
        // for x in [1, 2, 3, 4] {
        println!("x = {}", x);
    }

    println!("========i rev========");
    for i in (1..5).rev() {
        println!(" = {}", i);
    }

    let mut s = String::from("hello,world");
    s.push_str(" heige ");
    s.push_str("rust");
    println!("s = {}", s);

    let s = String::from("hello");
    let s2 = s; // 将s移动到s2
    println!("s2 = {}", s2);
    // println!("s = {}",s); // ^ value borrowed here after move
    // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
    // 当持有堆中数据值的变量离开作用域时，其值将 通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    let word = String::from("abc,hello");
    calculate(&word);
    println!("out s = {}", word);

    let mut word2 = String::from("hello");
    change(&mut word2); // 参数是一个可变引用类型
    println!("word2 = {}", word2); // word2 = hello,world
    // 可变引用原则：
    // 在特定作用域中的特定数据只能有一个可变引用,同时不能在拥有不可变引用的同时拥有可变引用；
    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止

    let mut r1 = String::from("abc");
    let r1 = &mut r1;
    // let r2 = &mut r1; // 这句会抛出panic
    println!("r1 = {}", r1);
    println!("dangle2 str = {}", dangle2());

    let s = String::from("hello world");
    println!("first word index: {}", first_word(&s)); // first word index: 5

    // slices切片类型
    let s = String::from("abcdefg");
    let s1 = &s[0..2];
    println!("s1 = {}", s1);
    println!("s2 = {}", &s[3..]); // defg

    let s = String::from("hello heige");
    println!("s3 = {}", first_word_str(&s)); // hello
    println!("s3 = {}", first_word_str2(&s[..])); // hello

    // 其他类型的 slice
    let a = [1, 2, 3, 4];
    println!("a[0..2]= {:?}", &a[0..2]); // a[0..2]= [1, 2] display格式化打印
}

fn calculate(s: &String) { // s是对于String引用，并不会获取s的所有权，s传递的一个不可变引用
    println!("s = {}", s) // 当s离开作用域后，不会丢弃它指向的数据
}

// change 改变s可变引用的值,可变引用修饰符&mut Type
fn change(s: &mut String) {
    s.push_str(",world");
    println!("current s = {}", s);
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用 ^ expected named lifetime parameter
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放 危险！

fn dangle2() -> String { // dangle 返回一个字符串
    let s = String::from("hello"); // s 是一个新字符串
    s
}

// 引用原则：
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用
// 引用必须总是有效的

// 返回第一个出现单词的位置
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 返回一个&str，它是一个字符串slice，&str 是一个不可变引用
// 它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；因为&str是一个不可变引用
// 字符串字面值就是 slice
fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 更加通用的函数，参数可以是字符串字面量，也可以是String,传递整个 String 的 slice切片类型
fn first_word_str2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}