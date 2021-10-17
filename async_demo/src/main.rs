use futures::executor::block_on;

async fn print_async(){
    println!("hello from print_async");
}

fn main() {
    println!("Hello, world!");
    let future = print_async();
    println!("hello main");
    block_on(future);

    // 异步闭包
    let closure = async {
        println!("Hello from async closure.");
    };
    println!("Hello from main again");
    block_on(closure);
}
