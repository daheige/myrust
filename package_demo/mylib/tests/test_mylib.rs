// 将单元测试统一放入tests目录中

#[test]
fn test_max(){
    println!("max(12,10) = {}",mylib::common::max(12,10));
}

#[test]
// #[should_panic(expected = "Guess value must in [1,100]")]
// 这个属性在函数中的代码 panic 时 会通过，而在其中的代码没有 panic 时失败
#[should_panic]
fn test_max_panic() {
    // assert_eq!(super::guess(12), true);
    assert_eq!(mylib::common::max(12,10), 123);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
