use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use typemap::{Key, TypeMap};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    // let state = Arc::new(RwLock::new(TypeMap::new()));
    // if let Some(init) = CMD_GROUP.init {
        // init(state.clone()).await;
    // }

    // let ctx = wab::Context {
        // state: state.clone(),
    // };
    // let mut args = HashMap::new();
    // args.insert(String::from("arg1"), wab::Argument::Integer(2));
    // args.insert(
        // String::from("arg2"),
        // wab::Argument::OptionalString(Some(String::from("va"))),
    // );
    // args.insert(String::from("arg3"), wab::Argument::Number(2.5));

    // let commands = (CMD_GROUP.build_commands)();
    // let _ = commands[0].run(ctx, args).await;
    
    // let ctx = wab::Context {
        // state: state.clone()
    // };
    // let mut args = HashMap::new();
    // args.insert(String::from("arg1"), wab::Argument::Integer(2));
    // args.insert(
        // String::from("arg2"),
        // wab::Argument::OptionalString(Some(String::from("va"))),
    // );
    // args.insert(String::from("arg3"), wab::Argument::Number(2.5));

    // let _ = commands[0].run(ctx, args).await;
    
    let mut bot = wab::Bot::builder().group(&CMD_GROUP).build();
    bot.register_interactions(String::from("a")).await;

    Ok(())
}

struct CmdState {
    x: i32,
}

impl Key for CmdState {
    type Value = Arc<RwLock<CmdState>>;
}

#[wab::box_async]
async fn init(state: Arc<RwLock<TypeMap>>) {
    let mut write = state.write().await;
    write.insert::<CmdState>(Arc::new(RwLock::new(CmdState {x: 5})));   
}

#[wab::group(
    category = "category here",
    commands(cmd, cmd2),
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
    ctx: wab::Context,
    mut arg1: i64,
    mut arg2: Option<String>,
    mut arg3: f64,
) -> wab::CommandResult {
    println!("boing1");
    let lock = ctx.get_state::<CmdState>().await;
    
    let count = {
        let mut counter = lock.write().await;
        counter.x = counter.x + 1;
        counter.x
    };
    println!("count: {}", count);

    Ok(())
}

#[wab::command(name = "cmd2 name", description = "cmd2 desc")]
pub async fn cmd2(ctx: wab::Context) -> wab::CommandResult {
    Ok(())
}
