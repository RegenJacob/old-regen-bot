use crate::discord::check_msg;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[group]
#[commands(wikisearch)]
pub struct Wiki;

#[command]
async fn wikisearch(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, "Comming Soon!");
    Ok(())
}
