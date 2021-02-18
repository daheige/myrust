fn main() {
    println!("Hello, world!");
    let s1 = gives_ownership();

    let mut s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2的所有权已经给了s3

    // println!("s2 = {}", s2); // 这里s2不能再使用了 ^^ value borrowed here after move
    s2 = takes_and_gives_back(s3);
    println!("s1 ={},s3 = {}", s1, s2);
    let l = calcute_length(&s2);
    println!("s2 len = {}", l);

    // 借用: &mut xxx
    // 当被借用之后，不能再使用之前的引用
    let l = calcute_length(&s2);
    println!("s len = {}", l);
    modify_str(&mut s2);
    println!("s2 = {}", s2);

    let mut s = String::from("hello");
    let l = calcute_length(&s);
    println!("s len = {}", l);
    modify_str(&mut s);
    println!("s = {}", s);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(str: String) -> String {
    str
}

// 引用： &
// 指向值的引用，但是不拥有它的所有权
// 当离开了作用域后，依然可以使用
fn calcute_length(s: &String) -> usize {
    s.len()
}

fn modify_str(s: &mut String) {
    s.push_str(" world");
}
