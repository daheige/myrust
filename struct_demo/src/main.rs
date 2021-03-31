// 定义结构体，这里采用Debug输出模式，可以打印变量{:?}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32); // 元祖类型的结构体，没有字段名字
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn dicz_area(d: (u32, u32)) -> u32 {
    d.0 * d.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle 给结构体定义方法和关联方法，用impl包含起来
impl Rectangle {
    // 如果想要在方法中改变调用方法的实例，需要将第一 个参数改为 &mut self
    // 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很 少见的
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can_hold 比较r1,r2
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 允许在 impl 块中定义 不 以 self 作为参数的函数。
    // 这被称为 关联函数（associated functions），因为它们与结构体相关联。
    // 即便如此它们仍是函数 而不是方法，因为它们并不作用于一个结构体的实例
    // 类似于静态方法
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// react_area 函数借用使用变量的值，但不获取所有权
fn react_area(r: &Rectangle) -> u32 {
    r.width * r.height
}

fn main() {
    println!("Hello, world!");
    let u = build_user(String::from("heige@hotmail.com"), String::from("heige"));

    println!("user is {:?}", u);

    let c = Color(1, 2, 3);

    println!("c is {:?}", c); // c is Color(1, 2, 3)

    println!("c.0 = {}", c.0);
    println!("{}", area(12, 10)); // 120
    println!("res = {}", dicz_area((12, 10))); // 120

    let r = Rectangle {
        width: 12,
        height: 12,
    };

    let r2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!("h = {},w = {}", r.height, r.width);
    println!("area is {}", react_area(&r));
    println!("area = {}", r.area());
    println!("r1 > r2 = {}", r.can_hold(&r2));

    let r = Rectangle::square(12);
    println!("r = {}", r.area());
}
