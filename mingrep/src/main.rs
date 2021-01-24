extern crate mingrep;

use mingrep::Config;
use std::env;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let c = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing args:{}", err);
        process::exit(1);
    });

    if let Err(err) = mingrep::run(c) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
