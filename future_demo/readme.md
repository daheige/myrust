# future
    async/.await 入门#

    async/.await 是 Rust 语言用于编写像同步代码一样的异步函数的内置工具。
    async 将一个代码块转化为一个实现了名为 Future 的特质（trait）的状态机。
    虽然在同步方法中调用阻塞函数会阻塞整个线程，但阻塞的 Futures 将让出线程控制权，
    允许其他 Futures 运行。
    
    要创建异步函数，可以使用 async fn 语法
```rust
async fn do_something() { // ... }
```
    async fn 返回的值是一个 Future，需要在执行者上运行才能起作用


# 参考

    https://learnku.com/docs/async-book/2018/async_await_primer/4788
