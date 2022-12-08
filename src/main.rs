mod context;
mod interactions;

use context::{Context, TwilightContext};
use dotenvy::{dotenv, var};
use futures::stream::StreamExt;
use std::{error::Error, sync::Arc};
use tracing::info;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    dotenv()?;

    info!("Initiating...");

    let token = var("DISCORD_TOKEN")?;
    let shards = var("SHARDS").unwrap_or_else(|_| 1.to_string()).parse::<u64>()?;

    let scheme = ShardScheme::Range {
        from: 0,
        to: 0,
        total: shards,
    };

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_MEMBERS;
    let (cluster, mut events) = Cluster::builder(token.clone(), intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    let cluster = Arc::new(cluster);

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = HttpClient::new(token);
    let resource_types = ResourceType::MESSAGE | ResourceType::MEMBER | ResourceType::ROLE;
    let cache = InMemoryCache::builder()
        .resource_types(resource_types)
        .build();

    let application_id = http
        .current_user_application()
        .await?
        .model()
        .await?
        .id;

    let user_id = http.current_user().await?.model().await?.id;

    let ctx = Arc::new(TwilightContext {
        http,
        cache,
        application_id,
        user_id,
    });

    interactions::handler::register_commands(Arc::clone(&ctx)).await?;

    while let Some((shard_id, event)) = events.next().await {
        ctx.cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, Arc::clone(&ctx)));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    ctx: Context,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::InteractionCreate(interaction) => {
            interactions::handler::interaction(ctx, (*interaction).0).await?
        }
        Event::ShardConnected(_) => {
            println!("Shard {} Connected to the Discord Gateway!", shard_id + 1);
        }
        _ => {}
    }

    Ok(())
}
