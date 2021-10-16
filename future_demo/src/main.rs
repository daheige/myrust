use futures::executor::block_on;

async fn hello(){
    println!("hello,world");
}

fn main() {
    let future = hello();
    block_on(future);
}
