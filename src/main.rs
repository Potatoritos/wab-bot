use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let command = car_build_cmd();

    let ctx = car::Context {};
    let mut args = HashMap::new();
    args.insert(String::from("arg1"), car::Argument::Int(2));
    args.insert(
        String::from("arg2"),
        car::Argument::OptionalString(Some(String::from("va"))),
    );

    let _ = command.run(ctx, args).await;

    Ok(())
}

#[car::command(
    name = "name here",
    description = "descrition here",
    parameter(name="arg1", description="boing")
)]
pub async fn cmd(ctx: car::Context, mut arg1: i64) -> car::CommandResult {
    println!("boing {}", arg1);
    Ok(())
}