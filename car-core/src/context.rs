use std::sync::Arc;
use tokio::sync::RwLock;
use typemap::{Key, TypeMap};

pub struct Context {
    pub state: Arc<RwLock<TypeMap>>,
}

impl Context {
    pub async fn get_state<T>(&self) -> <T as typemap::Key>::Value
    where
        T: Key,
        <T as Key>::Value: Clone,
    {
        let read = self.state.read().await;
        read.get::<T>().expect("a").clone()
    }
}
