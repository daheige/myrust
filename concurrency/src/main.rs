use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    println!("abc");
    let s = String::from("abc");
    println!("s = {}", s);

    /* thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 当main线程退出后，上面的thread线程不一定全部执行完毕
    for i in 1..5 {
        println!("main thread num:{}", i);
        thread::sleep(Duration::from_secs(1));
    }
    */

    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("main thread num:{}", i);
    }

    handler.join().unwrap(); // 调用join方法等待这一组线程执行完毕

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // move 关键字强制获取它使用的值的所有权
    let handler = thread::spawn(move || {
        for i in &v {
            println!("current v :{}", i);
        }
    });

    handler.join().unwrap();
    let (tx, rx) = mpsc::channel();
    // 通过克隆的方式创建多个生产者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let values = vec!["abc", "hi"];
        for val in values {
            tx1.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //  * 对于单个数据而言
    //  * let recv = rx.recv().unwrap();
    //  * println!("recv msg: {}", recv); // 接收消息 recv返回一个Result<T,E>形式
    for recv in rx {
        println!("recv msg: {}", recv);
    }
}
