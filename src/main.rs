use std::{env, error::Error, sync::Arc};
use tokio::sync::RwLock;
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::Intents;
use twilight_model::gateway::payload::incoming::MessageCreate;
use typemap_rev::{TypeMapKey, TypeMap};
use twilight_util::builder::InteractionResponseDataBuilder as ResponseBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;
    let resource_types = ResourceType::CHANNEL
        | ResourceType::EMOJI
        | ResourceType::GUILD
        | ResourceType::MEMBER
        | ResourceType::MESSAGE
        | ResourceType::PRESENCE
        | ResourceType::REACTION
        | ResourceType::ROLE
        | ResourceType::USER_CURRENT
        | ResourceType::USER
        | ResourceType::VOICE_STATE
        | ResourceType::STICKER;

    let bot = wab::Bot::builder().group(&CMD_GROUP).build();

    bot.run(
        env::var("WAB_TOKEN")?,
        env::var("WAB_APP_ID")?,
        intents,
        resource_types,
    )
    .await;

    Ok(())
}

struct CmdState {
    x: i32,
}

impl TypeMapKey for CmdState {
    type Value = Arc<RwLock<CmdState>>;
}

fn init(state: &mut TypeMap) {
    state.insert::<CmdState>(Arc::new(RwLock::new(CmdState { x: 5 })));
}

#[wab::event]
async fn message_create(event: &Box<MessageCreate>) {
    tracing::info!("{}", event.content);
}

#[wab::group(
    category = "category here",
    commands(cmd, cmd2),
    events(message_create),
    init = init
)]
pub struct CmdGroup;

#[wab::command(
    name = "name name2 name3",
    description = "description here",
    parameter(
        name = "arg1",
        description = "boing",
        min_value_int = 1,
        choice(name = "choice int", value_int = 2)
    ),
    parameter(
        name = "arg2",
        description = "description2",
        choice(name = "asdasdd", value_string = "value"),
        choice(name = "asda", value_string = "asdasfsfd"),
        min_length = 2,
        max_length = 40
    ),
    parameter(
        name = "arg3",
        description = "description3",
        min_value_number = "-1.02",
        max_value_number = 5.0
    )
)]
pub async fn cmd(
    ctx: wab::CommandContext,
    mut arg1: i64,
    mut arg2: String,
    mut arg3: Option<f64>,
) -> wab::CommandResult {
    println!("boing1");
    let lock = ctx.state.get::<CmdState>().await;

    let count = {
        let mut counter = lock.write().await;
        counter.x = counter.x + 1;
        counter.x
    };
    println!("count: {}", count);

    ctx.respond(ResponseBuilder::new().content(format!("count: {count}\n{arg1:?}, {arg2:?}, {arg3:?}")).build())
        .await;

    Ok(())
}

#[wab::command(name = "cmd2 name", description = "cmd2 desc")]
pub async fn cmd2(ctx: wab::CommandContext) -> wab::CommandResult {
    Ok(())
}
