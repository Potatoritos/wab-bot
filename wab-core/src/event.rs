use crate::BoxedFuture;
use twilight_gateway::{Event as EventData, EventType};

pub type EventFunction = fn(EventData) -> BoxedFuture<()>;

pub struct Event {
    pub kind: EventType,
    pub function: EventFunction,
}