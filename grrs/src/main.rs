use std::path as sPath;
// use std::env;
use structopt::StructOpt;
use std::fs;
use std::error;

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: sPath::PathBuf,
}

// 运行方式
// % cargo run -- abc test.txt
fn main() {
    // let pattern = env::args().nth(1).expect("no pattern given");
    // let path = env::args().nth(2).expect("no path given");
    // let args = Cli{
    //     pattern: pattern,
    //     path: sPath::PathBuf::from(path),
    // };

    // 读取命令行参数
    let args = Cli::from_args();
    println!("args:{:?}", args);

    let res = readfile(&args); // 返回值是一个Result,通过match模式匹配
    match res {
        Ok(_) => println!("read success"),
        Err(err) => println!("err: {}", err),
    }

    println!("====readfile2 read test.txt find heige====");
    let res = readfile2("test.txt".to_string(),"heige".to_string()); // 返回值是一个Result,通过match模式匹配
    match res {
        Ok(_) => println!("read success"),
        Err(err) => println!("err: {:?}", err),
    }
}

// 将错误进行装箱box处理
// 读取文件内容
fn readfile(args: &Cli) -> Result<(), Box<dyn error::Error>> {
    // let content = fs::read_to_string(&args.path).expect("could not read file");
    let content = fs::read_to_string(&args.path)?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // ()这是函数的默认返回值，意味着 “结果没问题，且没有内容”
    Ok(())
}

#[derive(Debug)]
struct CustomError(String);

fn readfile2(path: String,pattern: String) -> Result<(), CustomError> {
    // 格式化错误map_err
    let content = fs::read_to_string(&path).map_err(|err|CustomError(format!("read file `{}`,err:`{}",path,err)))?;
    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    // ()这是函数的默认返回值，意味着 “结果没问题，且没有内容”
    Ok(())
}
