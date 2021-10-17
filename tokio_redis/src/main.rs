use mini_redis::{client,Result};

#[tokio::main]
async fn main() ->Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello","daheige".into()).await?;

    // get key
    let result = client.get("hello").await?;
    println!("got value:{:?}",result);
    match result{
        Some(val)=> println!("value:{:?}",val),
        None=> println!("no value")
    }
    Ok(())
}
