use tokio::sync::RwLock;
use typemap_rev::{TypeMapKey, TypeMap};

pub struct State {
    pub storage: RwLock<TypeMap>
}
impl State {
    pub async fn get<T>(&self) -> <T as TypeMapKey>::Value
    where
        T: TypeMapKey,
        <T as TypeMapKey>::Value: Clone,
    {
        let read = self.storage.read().await;
        read.get::<T>().expect("a").clone()
    }  
}
