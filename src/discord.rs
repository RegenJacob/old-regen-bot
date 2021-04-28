use std::env;

use crate::fun::FUN_GROUP;
use crate::jacob::JACOB_GROUP;
use crate::voice::VOICE_GROUP;
use songbird::SerenityInit;

use serenity::client::{Context, EventHandler};
use serenity::http::Http;
use serenity::model::user::OnlineStatus;
use serenity::{
    async_trait,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{
        gateway::{Activity, Ready},
        prelude::Message,
    },
    Client, Result as SerenityResult,
};
use std::collections::HashSet;
use serenity::prelude::{TypeMapKey, Mutex};
use std::sync::Arc;
use serenity::client::bridge::gateway::ShardManager;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
//#[commands()]
struct General;

struct Handler;

pub async fn start() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not in environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".").owners(owners))
        .group(&GENERAL_GROUP)
        .group(&FUN_GROUP)
        .group(&JACOB_GROUP)
        .group(&VOICE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating Client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            ctx.shard
                .set_activity(Option::from(Activity::playing(format!(
                    "Auf Shard: {}",
                    ctx.shard_id
                ))));
            ctx.shard.set_status(OnlineStatus::Idle);

            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            println!(
                "{} is connected on shard {}/{}!",
                ready.user.name, shard[0], shard[1],
            );
        }
    }
}


pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        eprintln!("Error sending message: {:?}", why);
    }
}
