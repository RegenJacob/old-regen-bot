use std::env;

use crate::fun::FUN_GROUP;
use crate::voice::VOICE_GROUP;
use songbird::SerenityInit;

use serenity::{
    model::{
        gateway::{Ready, Activity},
        prelude::Message,
    },
    async_trait,
    Client,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    Result as SerenityResult,
};
use serenity::client::{EventHandler, Context};
use serenity::model::user::OnlineStatus;


#[group]
#[commands(help)]
struct General;

struct Handler;

#[tokio::main]
pub async fn start() {

    let token = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN not in environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP)
        .group(&FUN_GROUP)
        .group(&VOICE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating Client");


    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            ctx.shard.set_activity(Option::from(Activity::playing(format!("Auf Shard: {}", ctx.shard_id))));
            ctx.shard.set_status(OnlineStatus::Idle);

            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            println!(
                "{} is connected on shard {}/{}!",
                ready.user.name,
                shard[0],
                shard[1],
            );
        }
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e|{
            e.title("Commands");
            e.description("Das ist eine Liste der Commands");
            e.field("`.uff`", "Ja...", false);
            e.footer(|f| {
                f.text("Coded by RegenJacob");
                f.icon_url("https://cdn.discordapp.com/avatars/399247382916628480/6044316f7ab2bf0a9bba69c17736d08f.png");
                f
            })
        })
    }).await.unwrap();

    Ok(())
}

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
