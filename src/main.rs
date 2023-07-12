mod context;
mod interactions;
mod event;

use dotenvy::{dotenv, var};
use futures::stream::StreamExt;
use std::{error::Error, sync::Arc};
use tracing::{info, warn};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{stream::{self, ShardEventStream}, Config, Intents};
use twilight_http::Client as HttpClient;

use crate::context::TwilightContext;
use crate::event::process::ProcessEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    dotenv()?;

    info!("Initiating...");

    let token = var("DISCORD_TOKEN")?;

    let intents = Intents::GUILD_MESSAGES
        | Intents::MESSAGE_CONTENT
        | Intents::GUILD_MEMBERS
        | Intents::GUILDS;

    let config = Config::new(token.clone(), intents);
    let mut shards =
        stream::create_range(0..1, 1, config, |_, builder| builder.build()).collect::<Vec<_>>();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        warn!("Termination Signal received, Exiting...");
    });

    let http = HttpClient::new(token);
    let resource_types =
        ResourceType::CHANNEL | ResourceType::GUILD | ResourceType::MEMBER | ResourceType::ROLE;
    let cache = InMemoryCache::builder()
        .resource_types(resource_types)
        .build();

    let application_id = http.current_user_application().await?.model().await?.id;

    let user_id = http.current_user().await?.model().await?.id;

    let ctx = Arc::new(TwilightContext {
        http,
        cache,
        application_id,
        user_id,
    });

    interactions::handler::register_commands(Arc::clone(&ctx)).await?;

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard_id, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                let shard_id = shard_id.id().number() + 1;

                warn!("Shard {shard_id} Encountered an Error...");
                warn!(?source, "Error processing event...");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        ctx.cache.update(&event);

        tokio::spawn(event.process(Arc::clone(&ctx)));
    }


    Ok(())
}
