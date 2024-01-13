use tokio::sync::RwLock;
use typemap::{Key, ShareMap};

pub struct State {
    pub storage: RwLock<ShareMap>
}
impl State {
    pub async fn get<T>(&self) -> <T as typemap::Key>::Value
    where
        T: Key,
        <T as Key>::Value: Clone + Send + Sync,
    {
        let read = self.storage.read().await;
        read.get::<T>().expect("a").clone()
    }  
}