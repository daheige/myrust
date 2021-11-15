use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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

    // 线程handler
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

    // 通过channel方式消息传递
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
    // 从接收者中取出数据
    for recv in rx {
        println!("recv msg: {}", recv);
    }

    // 互斥锁mutex api
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 6;
    }

    println!("m = {:?}", m);

    // 在线程间共享mutex<T>
    // let counter = Mutex::new(0);
    /*
    for _ in 0..10 {
        let handler = thread::spawn(move || {
            // ^^^^^^^ value moved into closure here, in previous iteration of loop
            // counter已经移入线程执行的闭包中了，不能再次移入了
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }
     */

    // 通过使用智能指针 Rc<T> 来创建引用计数的值，以便拥有多所有者
    /*
    不幸的是， Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，
    并在每一个克隆被丢弃时减少计数。 Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。
    在 计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。
    我们所需要的是一个 完全类似 Rc<T> ，又以一种线程安全的方式改变引用计数的类型。
     */
    /*let counter = Rc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handler = thread::spawn(move || {
            // `Rc<Mutex<i32>>` cannot be sent between threads safely
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }*/

    // 原子引用计数 Arc<T>
    // 一个类似 Rc<T> 并可以安全的用于并发环境的类型
    // 原子性类型工作起来类似原始类型
    // 可以安全的在线程之间共享状态
    // 如果只是在单线程中对值进行操作，原子性提供的保 证并无必要，代码可以因此运行的更快
    println!("atomically reference counted");
    let mut handlers = vec![];
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 原子性克隆
        let handler = thread::spawn(move || {
            // `Rc<Mutex<i32>>` cannot be sent between threads safely
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    println!("result:{}", result);
}
