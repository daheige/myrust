extern crate mgrep as mingrep;

use mingrep::Config;
use std::env;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let c = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing args:{}", err);
        process::exit(1);
    });

    if let Err(err) = mingrep::run(c) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

/*
% cargo run body popm.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/mingrep body popm.txt`
Hello, world!
search for body,in file popm.txt
1.I’m nobody! Who are you?
2. Are you nobody, too?
5.6. How dreary to be somebody!
*/
