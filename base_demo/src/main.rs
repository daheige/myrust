#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    active: bool,
}

// 给结构体增加方法采用关键字impl
impl User {
    // self它代表调用该方法的结构体实例,这里是个不可变引用，具有读权限
    fn print_user(&self) {
        println!("current user name:{},age:{},active:{}", self.name, self.age, self.active);
    }

    // 这里self必须是可变引用才可以改变self上面的字段值，同时具有读写权限
    fn change(&mut self, name: String) {
        self.name = name;
        self.age = self.age + 1;
    }

    // 关联函数,返回新创建的实例对象User,这里可以使用Self表示
    // 相当于其他语言的静态方法
    fn build_user(name: &str, age: i32, active: bool) -> Self {
        let u = User {
            name: name.to_string(),
            age: age,
            // active:active,// 可以使用下面的方式简写
            active,
        };

        u
    }
}

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

    let u = User {
        name: String::from("heige"),
        age: 29,
        active: true,
    };

    println!("user is {:?}", u); // user is User { name: "heige", age: 29, active: true }

    // 创建一个可变的user
    let mut u1 = User {
        name: String::from("heige"),
        age: 29,
        active: true,
    };
    u1.name = String::from("daheige");
    println!("u.name: {}", u1.name);

    let u = User::build_user("daheige", 29, true);
    println!("name is {},age is {},active is {}", u.name, u.age, u.active);

    let u2 = User {
        name: String::from("heige"),
        age: 29,
        active: true,
    };

    // 利用其他结构体更新u2里面的字段
    let mut u3 = User {
        name: String::from("heige"),
        ..u2
    };

    println!("u3 is {:?}", u3); // 打印u3值
    println!("u3.age = {}", u3.age); // u3.age = 29
    println!("u3.active = {}", u3.active); // u3.active = true
    u3.print_user();
    u3.change(String::from("daheige"));
    u3.print_user(); // current user name:daheige,age:30,active:true

    let c = Color(12, 13);
    println!("c.0 = {}", c.0);

    let five = Some(5); // 声明一个有值的Option<T>
    // 在对 Option<T> 进行 T 的运算之前必须将其转换为 T
    // if let Some(T)匹配
    if let Some(i) = five {
        println!("i = {}", i);
    }

    let six = plus_one(five);
    println!("six = {:?}", six); // six = Some(6)
    println!("six = {:#?}", six); // 格式化打印
    /*
    six = Some(
        6,
    )
     */

    // 需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some成员保存的值的类型
    let absent_num: Option<i32> = None; // 声明一个空值的Option
    if let Some(i) = absent_num { // 采用if let 语法取得 Option<T>里面的值，T可以是任何类型
        println!("i = {}", i);
    } else {
        println!("none value");
    }

    let s = State::Active;
    show_state(s);

    // 通过if let Some匹配指定的值
    // if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值
    let s2 = Some(State::Active);
    if let Some(State::Active) = s2 {
        println!("active");
    }

    println!("===========struct type=========");
    let p = Point { x: 1, y: 1.2 };
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());
    let p2 = Point { x: 1.1, y: 1.2 };
    println!("distance is {}", p2.get_distance());

    let p3 = Point{x:"abc",y:"hello"};
    let p4 = p2.mixup(p3); // p3所有权被移入到了mixup里面去了，不能继续使用p3了
    // println!("p3.x = {},p3.y = {}",p3.x,p3.y); // ^^^^ value borrowed here after move
    println!("p4.x = {},p4.y = {}",p4.x,p4.y); // p4.x = 1.1,p4.y = hello

}

/*
 * Option<T> 类型是如何帮助你利用类型系统来避免出错的。
 当枚举值包含数据时，你可以根据需要处理多少情况来选择使用 match 或 if let 来获取并使用这些值
 要么包含值，要么没有
 <T> 它是一个泛型类型参数
 当枚举值包含数据时，你可以根据需要 处理多少情况来选择使用 match 或 if let 来获取并使用这些值
 match允许我们将一个值与一系列的模式相比较，并根据相匹 配的模式执行相应代码。
 模式可由字面值、变量、通配符和许多其他内容构成
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum State {
    Unknown,
    Active,
    Running,
    Stop,
}

fn show_state(s: State) { // match模式匹配
    match s {
        State::Unknown => println!("current state is {}", "unknown"),
        State::Active => println!("current state is {}", "active"),
        State::Running => println!("current state is {}", "running"),
        State::Stop => println!("current state is {}", "stop"),
        _ => println!("haha"),
    }
}

struct Color(i32, i32); // 元组结构体，没有字段名

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

// 结构体中定义的泛型 Point<T,U> 尖括号里面的是类型约定
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T,U>声明表明尖括号里面的类型是一个泛型参数，而不是具体类型
// 在 impl 之后声明 泛型 T
// 这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型
impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }

    // 混入其他类型
    fn mixup<V,W>(self,other:Point<V,W>) -> Point<T,W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

// 实现指定类型的方法
impl Point<f32, f32> {
    fn get_distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}