use std::env;

use crate::fun::FUN_GROUP;
use crate::owner::OWNER_GROUP;
use crate::voice::VOICE_GROUP;
use crate::wiki::WIKI_GROUP;
use songbird::SerenityInit;

use serenity::{
    async_trait,
    client::{
        bridge::gateway::GatewayIntents, bridge::gateway::ShardManager, Context, EventHandler,
    },
    framework::standard::{macros::group, StandardFramework},
    http::Http,
    model::{
        gateway::{Activity, Ready},
        prelude::Message,
    },
    prelude::TypeMapKey,
    Client, Result as SerenityResult,
};
use std::collections::HashSet;
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<serenity::prelude::Mutex<ShardManager>>;
}

#[group]
//#[commands()]
struct General;

struct Handler;

pub async fn start() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not in environment");

    if serenity::client::validate_token(&token).is_err() {
        println!("DISCOD_TOKEN is not even close to be a token");
        return;
    }

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let team = info.team.unwrap();
            let members = team.members;
            let mut owners = HashSet::new();

            for x in members {
                owners.insert(x.user.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    println!("{:?}", owners);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".").owners(owners))
        .group(&GENERAL_GROUP)
        .group(&FUN_GROUP)
        .group(&OWNER_GROUP)
        .group(&WIKI_GROUP)
        .group(&VOICE_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .register_songbird()
        .await
        .expect("Error creating Client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        println!("\nShutting down bot!");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start_autosharded().await {
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
            //ctx.shard.set_status(OnlineStatus::Idle);
            ctx.set_activity(Activity::competing("Idk")).await;

            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour. :(
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
