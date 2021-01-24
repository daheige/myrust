// 集成测试tests目录和src同一个层级
// 可以指定这个单元测试，cargo test --test add_two_test
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
