use twilight_cache_inmemory::InMemoryCache;
use twilight_model::id::{marker::ApplicationMarker, Id};

pub struct Client {
    pub http: twilight_http::Client,
    pub cache: InMemoryCache,
    pub application_id: Id<ApplicationMarker>
}

impl Client {
    
}