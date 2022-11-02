use std::sync::Arc;

use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client as HttpClient;
use twilight_model::id::{Id, marker::{UserMarker, ApplicationMarker}};

pub struct TwilightContext {
  pub http: Arc<HttpClient>,
  pub cache: InMemoryCache,
  pub application_id: Id<ApplicationMarker>,
  pub user_id: Id<UserMarker>
}

pub type Context = Arc<TwilightContext>;