use crate::discord::check_msg;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use std::time::Duration;
use base64::{encode, decode};


#[group]
#[commands(base64_decode)]
pub struct Utils;

#[command]
async fn base64_decode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Base 64 Decoder")
                .description(format!("{:?}", &decode(args.rest()).unwrap()[..]))
        })
    }).await?;
    Ok(())
}