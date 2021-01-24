pub mod network;

pub mod client;

// cargo test测试模块
#[cfg(test)]
mod tests {
    use super::client; // 调用相对于父模块
    #[test]
    fn it_works() {
        client::connect();
        assert_eq!(2 + 2, 4);
    }
}

/* 模块mod定义规则
如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs 的文件中。
如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件 中。

私有原则：
1. 如果一个项是公有的，它能被任何父模块访问
2. 如果一个项是私有的，它能被其直接父模块及其任何子模块访问
*/
