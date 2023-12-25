use std::sync::Arc;
use tokio::sync::RwLock;
use typemap::TypeMap;

pub struct Context {
    pub state: Arc<RwLock<TypeMap>>
}