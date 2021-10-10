use mylib::summary;
use mylib::summary::Summarizable;

#[derive(Debug)]
struct Post {
    author: String,
    title: String,
    content: String,
}

// 为类型Post实现 Summarizable trait（相当于post实现了接口Summarizable）
// Post实现了mylib包中的Summarizable的行为方法
impl Summarizable for Post {
    fn author(&self) -> String {
        format!("post author:{}", self.author)
    }

    fn summary(&self) -> String {
        format!("title:{},content:{}", self.title, self.content)
    }
}

// 为结构体Post再定义其他的方法
impl Post {
    fn get_post_title(&self) -> &String {
        &self.title
    }
}

/*
Trait Bound 语法
对泛型类型参数使用 trait。
我们 可以限制泛型不再适用于任何类型，编译器会确保其被限制为那些实现了特定 trait 的类型，
由此 泛型就会拥有我们希望其类型所拥有的功能。这被称为指定泛型的 trait bounds
*/
fn notify<T: Summarizable>(item: T) {
    println!("notify message:{}", item.summary());
}

/**
 * 对于拥有多个泛型类型参数的函数，每一个泛型都可以有其自己的 trait bounds。
 * 在函数名和参 数列表之间的尖括号中指定很多的 trait bound 信息将是难以阅读的，
 * 所以有另外一个指定 trait bounds 的语法，它将其移动到函数签名后的 where 从句中。所以相比这样写
 */
fn notify2<T>(t: T)
where
    T: Summarizable,
{
    println!("notify message:{}", t.summary());
}

fn main() {
    let headline = String::from("hello rust");
    let author = String::from("heige");
    let content = String::from("study rust,haha");
    let art = summary::Article::new(headline, author, content);
    println!("{:?}", art);
    println!("new art: {}", art.summary());
    println!("{}", art.author());
    // notify(art);

    let tweet = summary::Tweet {
        headline: String::from("cup"),
        location: String::from("China"),
        author: String::from("daheige"),
        content: String::from("rust"),
    };
    println!("{:?}", tweet);
    println!("new art: {}", tweet.summary());
    println!("{}", tweet.author());
    notify(tweet); // 调用泛型方法

    println!("Hello, world!");

    let post = Post {
        author: String::from("daheige"),
        title: String::from("hello,rust"),
        content: String::from("rust,123"),
    };
    print!("{}\n", post.get_post_title());
    print!("{}\n", post.summary());
    // notify(post);
    notify2(post);
}

/*
 * trait 和 trait bound 让我们使用泛型类型参数来减少重复，
 * 并仍然能够向编译器明确指定泛型 类型需要拥有哪些行为。
 * 因为我们向编译器提供了 trait bound 信息，它就可以检查代码中所用 到的具体类型是否提供了正确的行为。
 * 在动态类型语言中，如果我们尝试调用一个类型并没有实现的 方法，会在运行时出现错误。
 * Rust 将这些错误移动到了编译时，甚至在代码能够运行之前就强迫我 们修复错误。
 * 另外，我们也无需编写运行时检查行为的代码，因为在编译时就已经检查过了，
 * 这样相 比其他那些不愿放弃泛型灵活性的语言有更好的性能。
 */
