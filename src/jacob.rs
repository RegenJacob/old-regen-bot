use crate::discord::check_msg;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[group]
#[commands(eval)]
pub struct Jacob;

#[command]
#[help_available(false)]
async fn eval(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{}", msg.author.id);
    if msg.author.id != 399247382916628480 {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Keine Berechtigung!");
                    e.description("Nur <@399247382916628480> darf diesen Command benutzen!")
                })
            })
            .await?;
        println!("Ok");
        return Ok(());
    }

    msg.channel_id
        .send_message(&ctx.http, |m| m.content("Funny"))
        .await?;
    Ok(())
}
