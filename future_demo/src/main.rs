use futures::executor::block_on;

async fn hello(){
    println!("hello,world");
}

fn main() {
    let future = hello();
    block_on(future);

    // 在执行每一项任务时阻塞
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    println!("=====async exec task====");
    block_on(async_action());

}

struct Song;
async fn learn_song() -> Song { Song }
async fn sing_song(_: Song) {
    println!("sing song");
}
async fn dance() {
    println!("dance");
}

async fn learn_and_string(){
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_action(){
    let f1 = learn_and_string();
    let f2 = dance();

    // 允许其他任务并发执行
    // `join!` 类似于 `.await` ，但是可以等待多个 future 并发完成

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}
