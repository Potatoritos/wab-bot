use crate::BoxedFuture;
use std::sync::Arc;
use twilight_gateway::{Event as EventData, EventType};

pub type EventFunction = fn(Arc<EventData>) -> BoxedFuture<()>;

pub struct Event {
    pub kind: EventType,
    pub function: EventFunction,
}