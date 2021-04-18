// 在tests目录就可以建立任意的test文件，进行集成测试
extern crate adder_part;
#[test]
fn test_greet() {
    assert_eq!(1 + 1, 2);
    let result = adder_part::greet("abc");
    // assert!宏可以指定测试的条件表达式是否为true,如果不为true,第二个参数可以指定打印的信息，第三个参数是格式化字符串
    assert!(
        result.contains("abc"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
