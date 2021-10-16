use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    println!("longest(a,bc) = {}", longest("a", "bc"));

    /*
    创建了一个 ImportantExcerpt 的实例，它存放了变量 novel 所拥有的 String 的第一个句子的引用。
    novel 的数据在 ImportantExcerpt 实例创建之前就存在。另外，直到 ImportantExcerpt 离开作用域之后 novel
    都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的
     */
    let novel = String::from("call me ishmael.some years age...");
    let first_sentence = novel.split(".").next().expect("could not find a . ");
    let i = ImportantExcerpt{part: first_sentence};

    println!("i = {:?}",i);
    println!("nove: {}",novel);

    println!("hello,{}",longest_with_an_announcement("a","b",100));
}

// fn longest(x : &str,y : &str) -> &str{
//     if x > y{
//         x
//     }else{
//         y
//     }
// }
/*
^ expected named lifetime parameter
fn longest<'a>(x : &'a str,y : &'a str) -> &'a str{
  |           ^^^^     ^^^^^^^     ^^^^^^^     ^^^
  当我们定义这个函数的时候，并不知道传递给函数的具体值，所以也不知道到底是 if 还是 else 会被执行。
  我们也不知道传入的引用的具体生命周期，所以也就不能通过观察作用域来确定返回的引用是否总是有效。
  借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的。
  为了修复这个错误，我们将增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行 分析
 */

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}


// ImportantExcerpt 它存放了一个字符串 slice，这是一个引用。类似于泛型参数类型，
// 必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数。
// 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part: &'a str,
}

/*
ann 的类型是泛型 T ，它可以被放入任何实现了 where 从句中指定的 Display trait 的类 型。
这个额外的参数会在函数比较字符串 slice 的长度之前被打印出来，这也就是为什么 Display trait bound 是必须的。
因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一 尖括号列表中
 */
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}
