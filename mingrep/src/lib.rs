use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

// 将错误放入装箱中
// run 函数签名中声明成功类型返 回值是 () ，
// 这意味着需要将 unit 类型值包装进 Ok 值中。
// Ok(()) 一开始看起来有点奇 怪，不过这样使用 ()
// 是表明我们调用 run 只是为了它的副作用的惯用方式；它并没有返回 什么有意义的值
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search for {},in file {}", config.query, config.filename);
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    // println!("with text:\n{}", contents);
    for line in search(config.case_sensitive, &config.query, &contents) {
        eprintln!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(case_sensitive: bool, query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("case_sensitive:{}", case_sensitive);
    if case_sensitive {
        let mut results = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        return results;
    }
    search_case_insensitive(query, contents)
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

// 在结构体上定义方法
impl Config {
    // 关联函数
    // CASE_INSENSITIVE=1 cargo run boDy popm.txt
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough params");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
