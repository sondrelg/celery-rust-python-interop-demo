#![allow(non_upper_case_globals)]

use anyhow::Result;
use async_trait::async_trait;
use celery::prelude::*;
use env_logger::Env;
use structopt::StructOpt;
use tokio::time::{self, Duration};

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

// Demonstrates a long running IO-bound task. By increasing the prefetch count, an arbitrary
// number of these number can execute concurrently.
#[celery::task(max_retries = 2)]
async fn long_running_task(secs: Option<u64>) {
    let secs = secs.unwrap_or(10);
    time::sleep(Duration::from_secs(secs)).await;
}

// Demonstrates a task that is bound to the task instance, i.e. runs as an instance method.
#[celery::task(bind = true)]
fn bound_task(task: &Self) {
    // Print some info about the request for debugging.
    println!("{:?}", task.request.origin);
    println!("{:?}", task.request.hostname);
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "celery_app",
about = "Run a Rust Celery producer or consumer.",
setting = structopt::clap::AppSettings::ColoredHelp,
)]
enum CeleryOpt {
    Consume,
    Produce {
        #[structopt(possible_values = & ["add", "buggy_task", "bound_task", "long_running_task"])]
        tasks: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opt = CeleryOpt::from_args();

    let my_app = celery::app!(
        // broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into()) },
        broker = AMQPBroker { std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into()) },
        tasks = [
            add,
            buggy_task,
            long_running_task,
            bound_task,
        ],
        // This just shows how we can route certain tasks to certain queues based
        // on glob matching.
        task_routes = [
            "buggy_task" => "buggy-queue",
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10),
    ).await?;

    my_app.display_pretty().await;
    my_app.consume_from(&["celery", "buggy-queue"]).await?;

    my_app.close().await?;
    Ok(())
}