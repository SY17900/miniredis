use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);
    
    Ok(())
}

// use tokio::task::yield_now;
// use std::rc::Rc;

// #[tokio::main]
// async fn main() {
//     tokio::spawn(async {
//         let rc = Rc::new("hello");

//         // `rc` is used after `.await`. It must be persisted to
//         // the task's state.
//         yield_now().await;

//         println!("{}", rc);
//     });
// }
