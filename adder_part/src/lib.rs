pub mod larger;
pub fn plus(a: i32, b: i32) -> i32 {
    a + b * 2
}

pub fn greet(name: &str) -> String {
    format!("hello,{}", name)
}

pub fn guess(i: i32) -> bool {
    if i > 100 {
        panic!("i > 100");
    }

    true
}

// #[cfg(test)] 标注模块
// 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码， 而在运行 cargo build 时不这么做
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]标记
    #[test]
    fn test_plus() {
        println!("{}", "test plus start...");
        assert_eq!(super::plus(12, 1), 14);
        assert_ne!(super::plus(1, 3), 5);
    }

    #[test]
    fn test_react() {
        use super::larger; // 导入super上一层的larger模块
        let r1 = larger::Rectangle {
            width: 12,
            height: 32,
        };

        let r2 = larger::Rectangle {
            width: 10,
            height: 22,
        };

        println!("{}", "r1 > r2");
        assert_eq!(r1.can_hold(&r2), true);
    }

    #[test]
    fn test_greet() {
        let result = super::greet("abc");
        // assert!宏可以指定测试的条件表达式是否为true,如果不为true,第二个参数可以指定打印的信息，第三个参数是格式化字符串
        assert!(
            result.contains("abc"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // #[should_panic(expected = "Guess value must in [1,100]")]
    // 这个属性在函数中的代码 panic 时 会通过，而在其中的代码没有 panic 时失败
    #[should_panic]
    fn test_guess() {
        // assert_eq!(super::guess(12), true);
        assert_eq!(super::guess(120), true);
    }

    #[test]
    // 忽略函数#[ignore]
    #[ignore]
    fn test_add() {
        assert_eq!(1 + 1, 2);
    }
}

/*
cargo test
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test tests::it_works ... ok
test tests::test_plus ... ok
test tests::test_react ... ok

如果测试失败
// thread 'tests::test_greet' panicked at 'Greeting did not contain name, value was `hello,abc`', src/lib.rs:44:9\

// 单个线程运行
cargo test -- --test-threads=1

Rust 的测试库默认会捕获打印到标准输出的任何内容。例如，如果在测试中调用 println! 而测试通过了，我们将不会在终端看到 println!
可以指定参数来解决 以通过 --nocapture 参数来禁用
cargo test -- --test-threads=1 --nocapture

// 运行单个函数
cargo test test_react

*/
