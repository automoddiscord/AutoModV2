use futures::stream::StreamExt;

use twilight_cache_inmemory::{
    InMemoryCache, 
    ResourceType
};
use twilight_gateway::{
    cluster::{
        Cluster, 
        ShardScheme
    }, 
    Event, 
    Intents
};
use twilight_http::Client as HttpClient;

use tokio;

use std::{
    error::Error
};

#[path = "./handlers/cmds.rs"]
mod cmds;
use cmds::handle_command;

// use mongodb::{Client as mongoClient, options::ClientOptions};
// use trust_dns_resolver::config::ResolverConfig;

pub const PREFIX: &str = "!";




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    // let options = ClientOptions::parse_with_resolver_config(
    //     "",
    //     ResolverConfig::cloudflare(),
    // )
    // .await?;
    // let client = mongoClient::with_options(options)?;

    let token = "";

    let scheme = ShardScheme::Auto;

    let intents = Intents::GUILDS
        | Intents::GUILD_MEMBERS
        | Intents::GUILD_BANS
        | Intents::GUILD_EMOJIS
        | Intents::GUILD_INVITES
        | Intents::GUILD_MESSAGES
        | Intents::GUILD_MESSAGE_REACTIONS;

    let cluster = Cluster::builder(token, intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = HttpClient::new(token);

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .resource_types(ResourceType::GUILD)
        .build();

    let mut events = cluster.events();
    while let Some((shard_id, event)) = events.next().await {
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, http.clone(), cluster.clone()));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: HttpClient,
    cl: Cluster
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content.starts_with(&PREFIX) => {

            let content = &msg.content;
            let message = http.message(msg.channel_id, msg.id).await;
            let _result = handle_command(http, content.to_string(), message.unwrap().unwrap(), cl).await;
        }
        Event::ShardConnected(_) => {
            let bot = http.current_user().await?;
            println!(
                "Connected on shard {} as {}#{}",
                shard_id, bot.name, bot.discriminator
            );
        }
        _ => {}
    }

    Ok(())
}