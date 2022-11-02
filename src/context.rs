use std::sync::Arc;

use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ApplicationMarker, InteractionMarker, UserMarker},
    Id,
};

pub struct TwilightContext {
    pub http: HttpClient,
    pub cache: InMemoryCache,
    pub application_id: Id<ApplicationMarker>,
    pub user_id: Id<UserMarker>,
}

pub type Context = Arc<TwilightContext>;

pub struct CommandContext {
    pub twilight: Context,
    pub interaction_id: Id<InteractionMarker>,
    pub interaction_token: String,
}
