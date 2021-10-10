use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    // 闭包定义 参数放在管道||里面 {}是一个表达式
    // 闭包的 || {} 语法
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut res: i32 = x;
        res += 1;
        res += 1;
        res
    };
    assert_eq!(4, plus_two(2));
    let f = factory();
    let anwer = f(1);
    println!("anwer = 6 is {}", anwer == 6);
    assert_eq!(6, anwer);
    assert_eq!(6, f(1));

    generate_workout(12, 5);

    let add_one = |x: u32| -> u32 { x * 2 + 1 };

    println!("add_one(2) = {}", add_one(2));

    let ex = |x| x;
    let s = ex(String::from("hello")); // 这里调用后，闭包里面的参数类型就是string了
    println!("{}", s);
    /*
     *第一次使用 String 值调用 ex 时，编译器推断 x 和此闭包返回值的类型为 String。
     *接着这些类型被锁定进闭包 ex 中，如果尝试对同一闭包使用不同类型则会得到类型错误
     */
    // let m = ex(1); // expected struct `String`, found integer
    let m = ex(1.to_string()); // 调用to_string方法进行转化
    println!("{}", m);

    // ====================调用闭包方法============
    generate_workout2(12, 6);

    // 闭包中的move
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // 编译器自动推导z为 Vec<i32>类型

    // x所有权被移动到闭包中了，后续就不能再使用x
    // - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
    // println!("can not use x here {:?}", x); // - variable moved due to use in closure
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// 函数调用后闭包获得了被释放的内存环境
// 通过 Box 装箱，我们提供了一个已知大小的返回值，并允许它离开我们 的栈帧
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包定义
    let expensive_closure = |num| {
        println!("calculating slowly....");
        // 停顿2s
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today,do {} pushups!", expensive_closure(intensity));
        println!("next do {} situps!", expensive_closure(intensity));
        return;
    }

    if random_number == 3 {
        println!("task a break today! remember to stay hydrated!");
    } else {
        println!("today run for {} minutes!", expensive_closure(intensity));
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    // 闭包定义
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly....");
        // 停顿2s
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today,do {} pushups!", expensive_result.value(intensity));
        println!("next do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("task a break today! remember to stay hydrated!");
        } else {
            println!(
                "today run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// 定义结构体Cacher calculation 类型是一个函数闭包
// value 是一个option 存放函数T闭包执行的结果
struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    // 创建一个实例对象
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 如果 self.value 是 None，则会调用 self.calculation 中储存的闭包
    // 将结果保存到 self.value 以便将来使用，并同时返回结果值。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // 当函数没有调用就调用，然后将结果放入option 中缓存起来
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
