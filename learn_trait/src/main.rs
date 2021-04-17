fn main() {
    println!("Hello, world!");
    let list = vec![1, 2, 3, 4, 12, 123];
    println!("{}", larget_i32(&list));
    // 判断是否相等
    assert_eq!(larget_i32(&list), 123);

    // println!("{}", larget(&[1, 3, 4]));

    let p = Point { x: 1, y: 2 }; // 可以赋值不同的x,y
    println!("{:?}", p);
    let q = Point { x: 1.2, y: 3.4 };

    println!("x = {}", q.x);
    println!("get value of x = {}", q.get_x());

    let p = PointInfo {
        x: 1,
        y: 1.2,
        z: 123,
    };
    println!("p is {:?}", p); // p is PointInfo { x: 1, y: 1.2, z: 123 }

    let m = Option::Some(5);
    // if let Some(5) = m {
    //     println!("{}", 111);
    // }
    match m {
        Some(i) => println!("{}", i),
        None => println!("{}", "none"),
    };
    let m = Option_i32::Some(5);
    println!("{:?}", m);

    println!("==================");
    let list = vec![1, 2, 3, 4, 12, 123];
    println!("{}", larget(&list));
    // 判断是否相等
    assert_eq!(larget(&list), 123);
}
// larget_i32 i32类型的slice切片
fn larget_i32(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item
        }
    }

    max
}

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}
// 结构体泛型
// 这里结构体泛型，<T>类型必须是同一种类型才可以，否则就会panic
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 方法定义中的枚举数据类型,impl<T>TypeName<T> 这样的方式定义
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// 不同类型的结构体泛型定义，需要在<>中指定类型比如T,U两种不同的类型
#[derive(Debug)]
struct PointInfo<T, U> {
    x: T,
    y: U,
    z: T,
}

// 泛型,类型一般用T命名，fn_name<T>，这里的类型是放在函数后面，用<T>表示
// 采用trait bounds实现larget获得最大值
// Copy trait
// 以在 T 的 trait bounds 中增加 Copy！
// 只要传递 给 largest 的 slice 值的类型实现了 PartialOrd 和 Copy 这两个 trait
// 就可以调用larget泛型方法
fn larget<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item
        }
    }

    max
}
