const MAX_POINT: u32 = 1000;
fn main() {
    println!("Hello, world!");
    // let name: type = xxx变量定义 对于type可以自动推导出来
    let a = 12;
    println!("hello");
    println!("a = {}", a);
    let mut b: u64 = 123; // mut当前变量是可变化的
    println!("b = {}", b);
    b = 10;
    println!("b = {}", b);
    println!("max_point: {}", MAX_POINT);

    // boolean类型
    let is_open = true;
    let is_close = false;
    println!("is open = {},is close = {}", is_open, is_close);

    // 字符型 char是32位的
    let c = 'a';
    println!("c = {}", c);
    println!("c = {}",c);

    // 数字类型
    // i8,i16,i32,i64,u8,u16,u32,u64,f32,f64
    let d: i8 = -111;
    println!("d = {}", d);
    let f: f32 = 0.123;
    println!("f = {}", f);

    // 自适应类型isize,usize，跟操作系统有关系
    println!("max = {}", usize::max_value()); // 18446744073709551615

    // []数组类型 let arr: [type;size] = [xxx,xx];
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]= {}", arr[0]);
    show([1, 2, 3]);

    // 定义数组
    let arr2:[u64;3] = [1,2,3]; // [Type;N] 数组的定义
    println!("arr2[0]= {}",arr2[0]);

    // 元组
    let tup: (i32, f32, char) = (123, 12.56, 'a');
    // 打印元组
    println!("tup = {} {} {}", tup.0, tup.1, tup.2);

    let tup = (123, 12.21, 'a');
    println!("tup = {} {} {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("x = {},y= {},z= {}", x, y, z);

    let m = 1; // 语句，不返回值

    // let o = (let m1 = 1);
    println!("m = {}", m);

    // 表达式会计算一些值
    let m1 = {
        let x = 1;
        x + 1
    };

    println!("y = {}", m1); // m1 = 2

    show([1, 2, 234]);
}

// show 打印数组元素
fn show(arr: [u32; 3]) {
    for i in &arr {
        println!("current val: {}", i);
    }

    println!("a,b max: {}", max(1, 12));
}

// max a,b最大值 返回值 ->type
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}
