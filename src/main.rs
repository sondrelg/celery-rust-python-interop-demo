#![allow(non_upper_case_globals)]

use anyhow::Result;
use async_trait::async_trait;
use celery::prelude::*;
use env_logger::Env;

// This generates the task struct and impl with the name set to the function name "add"
#[celery::task]
fn say_hello() -> TaskResult<()> {
    println!("Hello from rust");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = celery::app!(
        broker = AMQPBroker { "amqp://127.0.0.1:5672" },
        tasks = [say_hello,],
        task_routes = ["rust_*" => "rust-queue",],
        prefetch_count = 2,
        heartbeat = Some(10),
    ).await?;

    app.display_pretty().await;
    app.consume_from(&["rust-queue"]).await?;
    app.close().await?;
    Ok(())
}