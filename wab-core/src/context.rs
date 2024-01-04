use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use typemap::{Key, ShareMap};

pub struct Context {
    pub state: Arc<RwLock<ShareMap>>,
    pub client: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
}

impl Context {
    pub async fn state<T>(&self) -> <T as typemap::Key>::Value
    where
        T: Key,
        <T as Key>::Value: Clone + Send + Sync,
    {
        let read = self.state.read().await;
        read.get::<T>().expect("a").clone()
    }
}
