/* rust常见的三种集合
vector 允许我们一个挨着一个地储存一系列数量可变的值
字符串（string）是一个字符的集合。我们之前见过 String 类型，不过在本章我们将深入了 解。
哈希 map（hash map）允许我们将值与一个特定的键（key）相关联。这是一个叫做 map 的 更通用的数据结构的特定实现
*/
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // 声明一个可变的集合
    let mut v: Vec<i32> = Vec::new(); // 创建一个Vec集合，里面的值是i32类型，类型位于<>中间
    v.push(12);
    v.push(123);
    println!("v is:{:?}", v);
    // 通过vec!宏创建一个集合 vector 在其离开作用域时会被释放
    let v = vec![1, 2, 4]; // 不可变集合
    println!("v is {:?}", v);
    let third = v[2];
    println!("v[3]= {}", third); // 4
    println!("v[0]= {:?}", v.get(0)); // get方法返回的是一个Option枚举

    let mut v = vec![1, 2, 3];
    // 不能在相同作用域中同时存在可变和不 可变引用的规则
    // let first = &v[0]; // - immutable borrow occurs here

    // println!("first is {}", first); // 调用下面的println!之后，first离开了作用域
    v.push(123);
    println!("v is {:?}", v);
    // 通过引用的方式（借用v)来对元素进行遍历
    for i in &v {
        println!("i = {}", i);
    }

    // 遍历可变 vector，从而修改vec里面元素的值
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50
    }
    println!("v is {:?}", v);

    // 使用枚举来储存多种类型
    /*
    Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多 少内存。
    第二个好处是可以准确的知道这个 vector 中允许什么类型。
    如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    使用 枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
    */
    let row = vec![
        Spread::Int(3),
        Spread::Text(String::from("blue")),
        Spread::Float(10.12),
    ];

    for i in &row {
        println!("i = {:?}", i);
    }
    /*
        i = Int(3)
    i = Text("blue")
    i = Float(10.12)
    */

    // 字符串 在rust底层是str，也就是字符串slice由字符组成，通常以&str借用的方式出现
    let mut s = String::new(); // 新建一个空的字符串
    s = s + "daheige"; // 连接字符串
    println!("s = {}", s);
    s.push_str(",hello"); //追加字符串
    println!("s = {}", s);
    s.push('1'); // 追加单个字符
    println!("s = {}", s);
    let s2 = "abc";
    s.push_str(&s2);
    println!("s2 is {}", s2);

    let s1 = String::from("abc");
    let s2 = String::from("efg");
    // +操作的签名是&str方式，所以这里借用了s2
    let s3 = s1 + &s2; // s1 has been moved here，后面再不能使用了 -- value moved here

    // println!("s1 is {},s3 is {}", s1, s3); // 这一句就会panic
    println!("s3 is {}", s3);

    let s1 = String::from("abc");
    let s2 = String::from("efg");
    let s3 = String::from("hello");
    // 字符串format!宏实现字符串的拼接
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s); // s is abc-efg-hello

    // Rust 的字符串不支持索引,因为字符串在rust中，String 是一个 Vec<u8> 的封装
    // 索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能
    // 因为 Rust 不得不检查从字符串的开头到索 引位置的内容来确定这里有多少有效的字符

    // 遍历字符串
    let s = String::from("abcef"); // 如果要操作单独的unicode标量值，可以用chars方法
    for c in s.chars() {
        println!("{}", c);
    }

    // 字符串并不简单
    // 总而言之，字符串还是很复杂的。不同的语言选择了不同的向程序员展示其复杂性的方式。
    // Rust 选 择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更 多的思考如何预先处理 UTF-8 数据。
    // 这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性， 不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误

    // map哈希
    // HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("yellow", 50);
    println!("blue score: {:?}", scores.get("blue")); // get 返回的是一个Option<V>

    // 遍历hash
    for (k, val) in &scores {
        println!("key = {},value = {}", k, val);
    }

    // 更新哈希 map 可以覆盖，也可以判断是否有
    scores.entry("red").or_insert(25); // 如果不存在就插入一个key/val
    println!("scores :{:?}", scores); // scores :{"yellow": 50, "red": 25, "blue": 10}

    // 根据旧的值更新
    let text = "hello red blue yellow hello yellow";
    let mut map = HashMap::new();
    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1 // 解引用的方式获得旧的值
    }

    println!("{:?}", map); // {"blue": 1, "red": 1, "hello": 2, "yellow": 2}
}

#[derive(Debug)]
enum Spread {
    Int(i32),
    Float(f64),
    Text(String),
}
