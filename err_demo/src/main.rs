use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    // 不可恢复的错误 panic!抛出，RUST_BACKTRACE=1 cargo run 可以看到完整的stack
    // panic!("crash and burn");
    // Resule与可恢复的错误
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("open file error:{:?}",err);
    //     },
    // };

    // 失败时panic简写  unwrap 会返回 Ok 中的值,如果失败了会调用panic!宏
    // let f = File::open("hello.txt").unwrap();

    // 另外一种方式可以用expect("xxx") 选择 panic! 的错误信息
    // 在多处使用 unwrap ，则需要花更多的时间来分析到底是哪一个 unwrap 造成了 panic，
    // 因为所有的 unwrap 调用都打印相同的信息
    // 调用 unwrap 或 expect 都是非常有道理的，比如说db,redis启动的时候连接不上去，这样的panic还是要抛出的
    // let f = File::open("hello.txt").expect("open file fail");
    let res: Result<String, io::Error> = read_name_from_file();
    println!("res is : {:?}", res);

    let res: Result<String, io::Error> = read_name_from_file2();
    println!("res is : {:?}", res);
}

// 传播错误的模式
fn read_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(err) => Err(err),
    // }

    // 传播错误的简写模式？
    // ? 只能被用于返回 Result 的函数
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式调用方式
/*
选择返回 Result 值的话，就将选择权交给了调用者，而不是 代替他们做出决定。
调用者可能会选择以符合他们场景的方式尝试恢复，或者也可能干脆就认为 Err 是不可恢复的，所以他们也可能会调用 panic!
并将可恢复的错误变成了不可恢复的错 误。因此返回 Result 是定义可能会失败的函数的一个好的默认选择
*/
fn read_name_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
