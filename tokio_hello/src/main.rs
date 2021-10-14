// extern crate tokio;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:6142";
    let mut stream = TcpStream::connect(addr).await?;

    println!("created stream");
    let result = stream.write(b"hello,world,daheige\n").await;
    println!("wrote to stream;success = {:?}", result.is_ok());
    Ok(())
}

/*
先运行nc
% nc -l 6142
hello,world,daheige

然后启动cargo run
created stream
wrote to stream;success = true
 */