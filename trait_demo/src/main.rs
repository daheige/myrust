// 结构体中的泛型 T,U是不同的类型
struct Point<T, U> {
    x: T,
    y: U,
}

// 泛型上的方法定义
// 必须在 impl 后面声明 T,U ，这样就可以在 Point<T,U> 上实现的方法中使用它了
impl<T, U> Point<T, U> {
    fn get_x_val(&self) -> &T {
        &self.x
    }
}

// 仅仅是x,y都是浮点型的时候，才支持，其他类型是不支持area方法的
impl Point<f32, f32> {
    fn area(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 对于enum中也可以使用泛型，并使用不同的类型

fn main() {
    println!("Hello, world!");
    let max = largest(&vec![1, 23, 234]);
    println!("max of vec is:{}", max);
    assert_eq!(max, 234);

    let num_list = vec![1, 2, 3, 4, 78];
    let max = largest(&num_list);
    println!("max of vec is:{}", max);
    assert_eq!(max, 78);

    let num_list = vec![1, 2, 3, 4, 78, 122];
    let max = largest_val(&num_list);
    println!("max of vec is:{}", max);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let max = largest_val(&char_list);
    println!("max of vec char is:{}", max);

    // 如果要比较其他类型的，找到最大值呢
    let p = Point { x: 1, y: 2 };
    println!("x = {},y = {}", p.x, p.y);
    let p = Point { x: 1.1, y: 2.1 };
    println!("x = {},y = {}", p.x, p.y);
    println!("p.x = {}", p.get_x_val());
    println!("p.area = {}", p.area());

    let p = Point { x: 1.1, y: 2 };
    println!("x = {},y = {}", p.x, p.y);

    let art = NewsArticle {
        headline: String::from("cup"),
        location: String::from("China"),
        author: String::from("daheige"),
        content: String::from("rust"),
    };
    println!("new art: {}", art.summary());
    println!("{}", art.author());
    notify(art);

    let art = Twiter {
        username: String::from("heige"),
        content: String::from("twiter hello"),
        reply: false,
        retweet: true,
    };
    println!("twiter art: {}", art.summary());
    println!("{}", art.author());
    notify(art);
}

// 会传递给函数的任何具体的 i32 值的 slice 切片类型
fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

// 定义泛型版本的vec集合里的最大值 下面的无法编译
// 只想对实现了 Copy 的类型调用这些代码，可以在 T 的 trait bounds 中增加 Copy
fn largest_val<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

// 定义trait里面的方法，可以默认实现方法summary
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("read more...")
    }

    fn author(&self) -> String;
}

pub struct Twiter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Twiter实现Summarizable summary方法
impl Summarizable for Twiter {
    fn summary(&self) -> String {
        format!(
            "username is:{},content:{},reply:{}",
            self.username, self.content, self.reply
        )
    }

    fn author(&self) -> String {
        format!("@author:{}", self.username)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现Summarizable summary方法
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!(
            "headline:{},location:{},author:{},content:{}",
            self.headline, self.location, self.author, self.content
        )
    }

    fn author(&self) -> String {
        format!("@author:{}", self.author)
    }
}

// notify函数名称，函数参数类型是Summarizable接口
// 通过泛型的方式，创建notify方法后面必须<T:XXX> 接着是()是参数列表
// 参数是接口类型trait
pub fn notify<T: Summarizable>(item: T) {
    println!("breaking news:{}", item.summary())
}
