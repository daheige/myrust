struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元祖结构体，没有命名字段的结构体
struct Point(i32, i32, i32);

// 定义结构体类型
// 结构体上没有Display实现方法，不能直接println!
// 通过注解的方式 增加注解来派生 Debug trait，并使用调试格式打印
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let u = User {
        username: String::from("daheige"),
        email: String::from("123@git.com"),
        sign_in_count: 1,
        active: true, // missing `active`结构体中的每个值，必须提前赋值，否则合理就会报错 missing `active`
    };

    // 借用u,引用的方式使用，否则- value moved here
    // 也就是函数调用后，就会值就会失效（离开了作用域，所有权没有了）
    print_user(&u);
    println!("{}", u.username);
    let p = Point(1, 2, 3); // 结构体元祖类型，类似于元祖
    println!("first value is: {},two value is: {}", p.0, p.1);
    let p1 = (12, 10);
    println!("area is: {}", area(p1));

    let r = Rect {
        width: 13,
        height: 12,
    };
    // println!("reac is:{}", r); // `Rect` cannot be formatted with the default formatter
    // println! 使用被称为 Display 的格式
    // println!("reac is:{:?}", r); // `Rect` cannot be formatted using `{:?}`
    println!("reac is:{:?}", r); // reac is:Rect { width: 13, height: 12 }

    /*通过{:#?}方式调试打印结构体
        reac is:Rect {
        width: 13,
        height: 12,
    }
    */
    println!("reac is:{:#?}", r);
    println!("reac area is:{}", rect_area(&r));
    println!("reac width is:{}", r.width);
    println!("call reac method area: {}", r.area());

    let r2 = Rect {
        width: 10,
        height: 11,
    };
    println!("r can hold r2: {}", r.can_hold(&r2));

    let sq = Rect::square(12);
    println!("square area is: {}", sq.area());
}

fn print_user(u: &User) {
    println!(
        "username: {},email: {},sign_count: {},ative: {}",
        u.username, u.email, u.sign_in_count, u.active
    );
}

// 使用元祖类型作为参数
fn area(d: (i32, i32)) -> i32 {
    d.0 * d.1
}

fn rect_area(r: &Rect) -> u32 {
    r.width * r.height
}

// 给结构体定义方法
// impl 是implementation 缩写,实现的意思
impl Rect {
    // 不可变地借用Rect实例对象，引用的方式，不改变self
    fn area(&self) -> u32 {
        // 定义area方法
        // self表示当前Rect实例对象
        println!("method call");
        println!("w:{} h:{}", self.width, self.height);
        self.width * self.height
    }

    // can_hold self是否包含other
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 定义关联函数，也就是相当于php语言里面的static方法
    // 结构体的关联方法，无需实例化结构体对象，返回一个结构体对象
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}
