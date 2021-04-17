use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();

    // 哈希 map 是同质的：所有的键必须是相同类型，值也 必须都是相同类型
    scores.insert("blue", 10);
    scores.insert("yello", 20);
    // scores.insert(String::from("red"), 20); // xpected `&str`, found struct `std::string::String`
    scores.insert("red", 123); // hash的值必须是一个类型

    println!("{}", scores["blue"]);
    println!("{}", scores["red"]);
    println!("{:?}", scores); //v{"blue": 10, "red": 123, "yello": 20}
    println!("===========");
    // 遍历map
    for (key, val) in &scores {
        println!("key = {} val = {}", key, val); // key = red val = 123
    }

    let teams = vec!["a", "b"];
    let scores = vec![12, 89];
    let smap: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("===========");
    for (key, val) in &smap {
        println!("key = {} val = {}", key, val); // key = b val = 89
    }

    let mut m = HashMap::new();
    m.insert(1, 12);
    m.insert(2, 234);
    // 访问1这个元素
    println!("{}", m[&1]);

    let mut m = HashMap::new();
    let key = String::from("a");
    m.insert(key, 12);
    m.entry(String::from("b")).or_insert(10); // 获得元素，如果没有就插入b=10

    // 遍历map
    for (k, v) in &m {
        println!("k = {},v = {}", k, v);
    }
}
