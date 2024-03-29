fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(12);

    for val in v1.iter() {
        println!("value: {}", val);
    }

    // 在迭代器的基础上调用map方法创建一个新的迭代器，接着调用collect方法消费新的迭代器并创建一个vec
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 is {:?}", v2); // v2 is [2, 3, 4, 5, 13]

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum is: {}", sum);

    let mut c = Counter::new();
    for val in c.next() {
        println!("{}", val);
    }
}

// cargo test -- --nocapture 禁用默认测试println!不输出的问题，这样就可以打印出来信息来
#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3, 4, 5];
    let total: i32 = v1.iter().sum();
    println!("total is {}", total);
}

#[test]
fn iter_enumerate() {
    use std::iter::Iterator;
    let v = &[1, 2, 3, 4, 5];
    for (key, value) in v.iter().enumerate() {
        println!("key:{} value:{}", key, value);
    }

    // 采用filter方法快速过滤
    let item = v.iter().filter(|&x| *x % 2 == 0).nth(2); // nth(x)表示第几个索引位置上的元素
    println!("{:?}", item);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// shoes_in_my_size 对指定的vec shoe进行过滤
// 函数体中调用了 into_iter 来创建一个获取 vector 所有权的迭代器
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size >= shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 15,
            style: String::from("boot"),
        },
        Shoe {
            size: 16,
            style: String::from("joho"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 12);
    println!("{:?}", in_my_size);
}
/*
% cargo test filters_by_size -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/iter_demo-556d8edb1ea5f382)

running 1 test
[Shoe { size: 13, style: "sandal" }, Shoe { size: 15, style: "boot" }, Shoe { size: 16, style: "joho" }]
test filters_by_size ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
*/

// 实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    // 初始化counter实例
    fn new() -> Self {
        Self { count: 0 }
    }
}

// 实现 Iterator trait 来创建自定义迭代器
// 实现迭代器上的next方法
// 标准库中其他方法，默认已经实现了，不需要再次实现
// 标准库则提供了其它调用 next 的方法的默认实现
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn call_next_directly() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}

#[test]
fn using_other_iterator_counter() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
