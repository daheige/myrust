// use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration; // 多个生产者-单个消费者模式N:1 // mutex互斥锁

fn main() {
    println!("Hello, world!");
    // 为了创建一个新线程，需要调用 thread::spawn 函数并传递一个闭包
    thread::spawn(|| {
        for i in 1..10 {
            println!("number = {} from spawned thread", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        // thread::sleep 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行。
        // 这些线程可能会轮流运行，不过并不保证如此：这依赖操作系统如何调度线程。
        thread::sleep(Duration::from_millis(1));
    }
}

/*
可以通过将 thread::spawn 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。
thread::spawn 的返回值类型是 JoinHandle。JoinHandle 是一个拥有所有权的值，
当对其调用 join 方法时，它会等待其线程结束。
*/
#[test]
fn test_thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number = {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

/*
% cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
hi number 1 from main thread
number = 1 from spawned thread
hi number 2 from main thread
number = 2 from spawned thread
hi number 3 from main thread
number = 3 from spawned thread
hi number 4 from main thread
number = 4 from spawned thread
number = 5 from spawned thread
number = 6 from spawned thread
number = 7 from spawned thread
number = 8 from spawned thread
number = 9 from spawned thread
test test_thread_join ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
*/

#[test]
fn test_thread_join2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number = {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); // 等待线程执行完毕，再执行后面的操作
    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/*
主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现
% cargo test test_thread_join2 -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
number = 1 from spawned thread
number = 2 from spawned thread
number = 3 from spawned thread
number = 4 from spawned thread
number = 5 from spawned thread
number = 6 from spawned thread
number = 7 from spawned thread
number = 8 from spawned thread
number = 9 from spawned thread
hi number 1 from main thread
hi number 2 from main thread
hi number 3 from main thread
hi number 4 from main thread
test test_thread_join2 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.02s
*/

#[test]
fn test_thread_join3() {
    let v = vec![1, 2, 3, 4];
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("number = {} from spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    //     // 报错 ^^ may outlive borrowed value `v`
    //     println!("vec = {:?} from spawned thread", v);
    // });

    // 闭包中通过move获得v所有权
    // 将 v 的所有权移动到新建线程
    let handle = thread::spawn(move || {
        println!("vec = {:?} from spawned thread", v);
    });

    handle.join().unwrap(); // 等待线程执行完毕，再执行后面的操作
}

// ==================通过channel通道消息传递==============
#[test]
fn test_mpsc() {
    // 返回值是一个元祖
    let (tx, rx) = mpsc::channel();
    // 开辟线程模式发送数据
    std::thread::spawn(move || {
        let val = String::from("hello,world");
        tx.send(val).expect("send msg fail");

        // tx.send 发送 val 到通道中之后将其打印出来,这种方式是不行的，因为数据已经被发送了，如果再使用，可能导致异常
        // 这是rust语言中不允许的操作
        // println!("{}", val);
    });

    // 在线程中接收数据
    let recv = rx.recv().expect("recv msg fail");
    println!("got msg {}", recv);
}

/*
% cargo test test_mpsc -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
got msg hello,world
test test_mpsc ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
*/

/*
通道的接收端有两个有用的方法：recv 和 try_recv。这里，我们使用了 recv，
它是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值。
一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。

try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，
在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。

出于简单的考虑，这个例子使用了 recv；主线程中除了等待消息之外没有任何其他工作，所以阻塞主线程是合适的。
*/

/*
发送多个值并观察接收者的等待
*/
#[test]
fn test_more_producter() {
    // mpsc是 multiple producer, single consumer 的缩写
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("heige"),
        ];

        for val in vals {
            tx.send(val).expect("send value fail");
        }
    });

    // 主线程会阻塞，打印接收到的值
    for recv in rx {
        println!("got value: {}", recv);
    }
}

/*
% cargo test test_more_producter -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
got value: hello
got value: world
got value: heige
test test_more_producter ... ok
*/

// 通过克隆的方式创建多个生产者
#[test]
fn test_more_producter2() {
    // mpsc是 multiple producer, single consumer 的缩写
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // 克隆一个生产者

    // 这样就会有两个线程，每个线程将向通道的接收端发送不同的消息
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("heige"),
        ];

        for val in vals {
            tx.send(val).expect("send value fail");
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("rust"),
            String::from("golang"),
            String::from("js"),
        ];

        for val in vals {
            tx1.send(val).expect("send value fail");
        }
    });

    // 主线程会阻塞，打印接收到的值
    for recv in rx {
        println!("got value: {}", recv);
    }
}

/*% cargo test test_more_producter2 -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
got value: rust
got value: golang
got value: js
got value: hello
got value: world
got value: heige
test test_more_producter2 ... ok
 */

/*在某种程度上，任何编程语言中的通道都类似于单所有权，因为一旦将一个值传送到通道中，
* 将无法再使用这个值。共享内存类似于多所有权：多个线程可以同时访问相同的内存位置
mutex 互斥锁机制
 */

#[test]
fn test_mutex() {
    // Mutex<i32> 并不是一个 i32，所以 必须 获取锁才能使用这个 i32 值
    // lock 调用 返回 一个叫做 MutexGuard 的智能指针。
    // 这个智能指针实现了 Deref 来指向其内部数据；
    // 其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
    let m = Mutex::new(5); // 创建一个类型T的互斥锁
    {
        let mut num = m.lock().expect("lock fail");
        *num = 6; // 改变m的值
    }

    println!("m = {:?}", m);
}
/*
% cargo test test_mutex -- --nocapture
   Compiling spawn_demo v0.1.0 (/Users/heige/web/rust/spawn_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.83s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
m = Mutex { data: 6 }
test test_mutex ... ok
*/

// #[test]
// fn test_mutex_count() {
//     // ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
//     let counter = Mutex::new(0);
//     let mut handlers = vec![];
//     // 开辟10个线程处理push
//     for _ in 0..10 {
//         // ^^^^^^^ value moved into closure here, in previous iteration of loop
//         // 错误信息表明 counter 值在上一次循环中被移动了
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handlers.push(handle);
//     }

//     for handle in handlers {
//         handle.join().unwrap();
//     }

//     println!("result: {}", *counter.lock().unwrap());
// }

// 通过使用智能指针 Rc<T> 来创建引用计数的值，以便拥有多所有者
// #[test]
// fn test_mutex_count() {
//     let counter = Rc::new(Mutex::new(0)); // 通过Rc<T>方式创建的值，拥有多个所有者
//     let mut handlers = vec![];
//     // 开辟10个线程处理push
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         // `Rc<Mutex<i32>>` cannot be sent between threads safely
//         /*
//          * Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。
//          * Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。
//          * 在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。
//          * 我们所需要的是一个完全类似 Rc<T>，又以一种线程安全的方式改变引用计数的类型。
//          */
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handlers.push(handle);
//     }

//     for handle in handlers {
//         handle.join().unwrap();
//     }

//     println!("result: {}", *counter.lock().unwrap());
// }

// 原子引用计数 Arc<T>
// Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），
// 所以这是一个原子引用计数（atomically reference counted）类型
// Arc<T> 和 Rc<T> 有着相同的 API，所以修改程序中的 use 行和 new 调用
// 使用 Arc<T> 包装一个 Mutex<T> 能够实现在多线程之间共享所有权

#[test]
fn test_mutex_count() {
    let counter = Arc::new(Mutex::new(0)); // 通过 Arc<T>方式创建的值，拥有多个所有者
    let mut handlers = vec![];
    // 开辟10个线程处理push
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}

/*
% cargo test test_mutex_count -- --nocapture
   Compiling spawn_demo v0.1.0 (/Users/heige/web/rust/spawn_demo)
    Finished test [unoptimized + debuginfo] target(s) in 0.76s
     Running unittests (target/debug/deps/spawn_demo-816ea47573223c1a)

running 1 test
result: 10
test test_mutex_count ... ok
*/

/*使用 Sync 和 Send trait 的可扩展并发 总结
通过 Send 允许在线程间转移所有权

Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是Send 的，
不过有一些例外，包括 Rc<T>：这是不能 Send 的，因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，
这两个线程都可能同时更新引用计数。为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。

因此，Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 Rc<T> 在线程间发送。
当尝试这么做的时候，会得到错误 the trait Send is not implemented for Rc<Mutex<i32>>。
而使用标记为 Send 的 Arc<T> 时，就没有问题了

Sync 允许多线程访问

Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
换一种方式来说，对于任意类型 T，如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的，
这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。
智能指针 Rc<T> 也不是 Sync 的，出于其不是 Send 相同的原因

无畏并发

Rust 提供了用于消息传递的通道，和像 Mutex<T> 和 Arc<T> 这样可以安全的用于并发上下文的智能指针。
类型系统和借用检查器会确保这些场景中的代码，不会出现数据竞争和无效的引用。一旦代码可以编译了，
我们就可以坚信这些代码可以正确的运行于多线程环境，而不会出现其他语言中经常出现的那些难以追踪的 bug。
并发编程不再是什么可怕的概念：无所畏惧地并发吧！
*/
