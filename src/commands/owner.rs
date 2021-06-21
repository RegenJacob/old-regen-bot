//! Owner Commands and Infos about the Bot.
//! the eval command allows the owner or an team member to run code
//! Info command shows info's about the bot

use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use std::collections::HashSet;
use std::convert::Infallible;

#[group]
#[commands(eval, info, set_status)]
pub struct Owner;

#[command]
#[help_available(false)]
#[owners_only]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| m.content(args.rest()))
        .await?;

    Ok(())
}

#[command]
#[help_available(false)]
#[owners_only]
async fn set_status(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match args.clone().single::<String>().unwrap().as_str() {
        "online" => ctx.online().await,
        "offline" => ctx.invisible().await,
        "dnd" => ctx.dnd().await,
        "idle" => ctx.idle().await,
        _ => {}
    }

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let (owners, bot_id) = match &ctx.http.get_current_application_info().await {
        Ok(info) => {
            let team = info.team.as_ref().unwrap();
            let members = &team.members;
            let mut owners = HashSet::new();

            for x in members {
                owners.insert(x.user.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Info über den bot:");
                e.field("Entwickler:", format!("{:?}", owners), true);
                e.field("Bot ID", bot_id.to_string(), true);
                e.footer(|f| f.text("Version 1.0.0"))
            })
        })
        .await?;
    Ok(())
}
