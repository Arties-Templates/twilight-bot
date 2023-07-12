use std::fmt::Debug;

use twilight_cache_inmemory::UpdateCache;
use twilight_model::gateway::{event::Event as GatewayEvent, payload::incoming};

use crate::context::Context;
use async_trait::async_trait;
use tracing::{error, info, trace};

// Derived from:
// https://github.com/baptiste0928/raidprotect/blob/main/raidprotect/src/event/process.rs

#[async_trait]
pub trait ProcessEvent: Sized {
    async fn process(self, ctx: Context);
}

macro_rules! process_events {
  ($self:ident, $context:ident => $( $event:path ),+ ) => {
    match $self{
    $(
      $event(event) => event.process($context).await,
    )+
    event => trace!(kind = event.kind().name(), "Unprocessed event")
  }
}
}

async fn process_cache_event<E: UpdateCache + Debug>(event: E, ctx: Context) {
    event.update(&ctx.cache)
}

macro_rules! process_cache_events {
  ( $( $event:ident ),+ ) => {
    $(
      #[async_trait]
      impl ProcessEvent for incoming::$event {
        async fn process(self, ctx: Context) {
          process_cache_event(self, ctx).await;
        }
      }
    )+
  };
}

process_cache_events! {
  GuildCreate,
  GuildDelete,
  UnavailableGuild,
  GuildUpdate,
  ChannelUpdate,
  ChannelDelete,
  ChannelCreate,
  MemberUpdate
}

#[async_trait]
impl ProcessEvent for GatewayEvent {
    async fn process(self, ctx: Context) {
        use GatewayEvent::*;

        process_events! { __self, ctx =>
          InteractionCreate,
          Ready
        }
    }
}

#[async_trait]
impl ProcessEvent for incoming::Ready {
    async fn process(self, _ctx: Context) {
        let shard = self.shard;

        if let Some(shard) = shard {
            info!("Shard {} Connected to the Discord Gateway!", shard.number());
        }

        info!("A Shard connected to the Discord Gateway!");
    }
}

#[async_trait]
impl ProcessEvent for incoming::InteractionCreate {
    async fn process(self, ctx: Context) {
        match crate::interactions::handler::interaction(ctx, self.0).await {
            Ok(_) => {}
            Err(why) => error!("Failed to handle Interaction!{why:?}"),
        }
    }
}
