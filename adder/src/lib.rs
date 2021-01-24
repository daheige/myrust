pub fn add_two(x: i32) -> i32 {
    x + x
}

// #[cfg(test)] 告诉rust仅仅在cargo test进行build，运行的时候不走这个test
// 以下是单元测试的写法
#[cfg(test)]

// cargo test -- --test-threads=3 指定测试的线程个数
mod tests {
    //  这是一个测试函数
    // cargo test it_works 指定某个函数进行测试
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); // 测试相等
        assert_ne!(1 + 3, 2); // 不想等
        let b = true;
        assert!(b); // 测试bool值
    }

    #[test]
    fn contain_name() {
        let res = "abc efg";
        assert!(res.contains("abc"));
    }

    #[test]
    #[ignore] // 忽略某个函数测试
    fn another() {
        panic!("make this test fail");
    }
}
