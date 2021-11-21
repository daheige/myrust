use std::ops::Add;

// 运算符+重载
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let p1 = Point { x: 1, y: 123 };
    let p2 = Point { x: 1, y: 123 };
    println!("p1+p2:{:?}", p1 + p2); // p1+p2:Point { x: 2, y: 246 }

    let p1 = Point { x: 1, y: 123 };
    let p2 = Point { x: 1, y: 123 };
    println!("p2 = {:?}", p2);
    println!("p1+p2 = {:?}", p1 + p2);
    // println!("p1.x = {}", p1.x); // 这里p1所有权已经被p1+p2加操作移动了所有权，这里就不能继续使用了
}
