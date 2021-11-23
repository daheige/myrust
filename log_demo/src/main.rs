use log::{debug, error, info};
use std::env;

// 日志级别从高到低 error > warn > info > debug > trace

// 其他强大的日志包 log4rs
fn main() {
    // env::set_var("RUST_LOG", "log_demo=debug"); // 通过调用函数设置环境变量

    env_logger::init(); // 日志初始化
    println!("Hello, world!");
    info!("abc");
    debug!("123");
}

// RUST_LOG=path::to_module=log_leve 多个用逗号隔开
// 比如 RUST_LOG=log_dem=info,abc=debug

//  % RUST_LOG=log_demo=debug cargo run
/*Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target/debug/log_demo`
Hello, world!
[2021-11-22T14:32:10Z INFO  log_demo] abc
[2021-11-22T14:32:10Z DEBUG log_demo] 123*/
