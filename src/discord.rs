use std::env;

use crate::commands::fun::FUN_GROUP;
use crate::commands::owner::OWNER_GROUP;
use crate::commands::voice::music::MUSIC_GROUP;
use crate::commands::wiki::WIKI_GROUP;
use crate::commands::utils::UTILS_GROUP;
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
use serenity::model::gateway::{ActivityButton, ActivityType};
use serenity::model::prelude::OnlineStatus;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<serenity::prelude::Mutex<ShardManager>>;
}

#[group]
//#[commands()]
struct General;

struct Handler;

pub async fn start() {
    println!("Starting Bot!");
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not in environment");

    if serenity::client::validate_token(&token).is_err() {
        println!("DISCORD_TOKEN is not a token lol ur dumb");
        return;
    }

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
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
        .group(&UTILS_GROUP)
        .group(&MUSIC_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(*bot_id.as_u64())
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
            ctx.shard.set_status(OnlineStatus::Idle);
            
            tokio::spawn(async move {
                loop {
                    ctx.set_activity(Activity::playing("keine Spiele!")).await;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    ctx.set_activity(Activity::competing("in nichts")).await;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                    ctx.set_activity(Activity::listening("dir zu!")).await;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                }
            });

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
