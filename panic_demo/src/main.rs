use std::fs::File;
use std::io;
// use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    // say();
    // open();
    // let f = File::open("hello.txt").unwrap();

    //  unwrap 调用 panic! 时提 供的错误信息

    // expect打印期望的提示信息
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("{}", "hello");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // 将错误给调用方去处理
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写?
// 有错误提前返回
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 可以在 ? 之后直接使用链式方法 调用来进一步缩短代码
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// fn open() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(ref err) if err.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => {
//                 panic!("open file error: {:?}", e);
//             }
//         },
//     };
// }

fn say() {
    panic!("exec panic");
}

// RUST_BACKTRACE=1 cargo run
// 运行并打印堆栈信息

// open file error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
