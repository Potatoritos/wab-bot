use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let command = car_builder_cmd();

    let ctx = car::Context {};
    let mut args = HashMap::new();
    args.insert(String::from("arg1"), car::Argument::Int(2));
    args.insert(
        String::from("arg2"),
        car::Argument::OptionalString(Some(String::from("va"))),
    );
    args.insert(String::from("arg3"), car::Argument::Number(2.5));

    // let _ = command.run(ctx, args).await;

    Ok(())
}

#[car::group(category = "category here", commands(cmd, cmd2))]
pub struct CmdGroup;

#[car::command(
    name = "name here",
    description = "description here",
    parameter(
        name = "arg1",
        description = "boing",
        min_value_int = 1,
        choice_int(name = "choice int", value = 2)
    ),
    parameter(
        name = "arg2",
        description = "description2",
        choice_string(name = "asdasdd", value = "value"),
        choice_string(name = "asda", value = "asdasfsfd"),
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
    ctx: car::Context,
    mut arg1: i64,
    mut arg2: Option<String>,
    mut arg3: f64,
) -> car::CommandResult {
    println!("boing {}", arg1);
    Ok(())
}

#[car::command(name = "cmd2 name", description = "cmd2 desc")]
pub async fn cmd2(ctx: car::Context) -> car::CommandResult {
    Ok(())
}
