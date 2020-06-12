mod handlers;
mod helpers;
mod structures;
mod commands;

use twilight::{
    gateway::{Cluster, ClusterConfig, Event},
    http::Client as HttpClient,
};
use futures::StreamExt;
use std::{env, error::Error};
use helpers::{
    command_utils,
    database_helper,
    credentials_helper
};
use handlers::command_handler;
use structures::Context;
use std::{sync::Arc, collections::HashMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let mut data: HashMap<String, String> = HashMap::new();
    let creds = credentials_helper::read_creds(args[1].to_string()).unwrap();

    let http = HttpClient::new(&creds.bot_token);
    let pool = database_helper::obtain_pool(creds.db_connection).await?;
    data.insert(String::from("default_prefix"), creds.default_prefix);

    let cluster_config = ClusterConfig::builder(&creds.bot_token).build();
    let cluster = Cluster::new(cluster_config);
    cluster.up().await?;

    let mut events = cluster.events().await;

    while let Some(event) = events.next().await {
        let ctx = command_utils::create_context(http.clone(), pool.clone(), data.clone()).await;
        tokio::spawn(handle_event(event, ctx));
    }

    Ok(())
}

async fn handle_event(event: (u64, Event), ctx: Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        (id, Event::Ready(_)) => {
            println!("Connected on shard {}", id);
        }
        (_, Event::MessageCreate(msg)) => {
            let default_prefix = ctx.data.get("default_prefix").unwrap();

            if &msg.content[..1] == default_prefix.as_str() {
                match command_handler::handle_command(msg, ctx).await {
                    Ok(()) => {},
                    Err(error) => println!("Command Error!: {}", error)
                };
            }
        }
        _ => {}
    }

    Ok(())
}
