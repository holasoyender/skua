use skua::framework::standard::macros::command;
use skua::framework::standard::CommandResult;
use skua::model::prelude::*;
use skua::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}
