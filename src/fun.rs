use crate::discord::check_msg;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{
    command,
    group
};
use serenity::framework::standard::Args;
use std::time::Duration;
use uwuifier::uwuify_str_sse;

fn uwuify(input: &str) -> String {
    uwuify_str_sse(input)
}

#[group]
#[commands(mcstatus, uwu, panzer, unsee)]
pub struct Fun;

#[command]
async fn mcstatus(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let ip = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Server IP ist ungültig").await);

            return Ok(());
        },
    };

    let a = ip.clone();

    let (latency, status) = tokio::task::block_in_place(move || {
        let (latency, status) = mcping::get_status(&*a, Duration::from_secs(10)).expect("Not a Server");
        (latency, status)
    });

    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e|{
            e.title(format!("Info über {}", ip.clone()));
            e.description(status.description.text());
            e.thumbnail(format!("https://eu.mc-api.net/v3/server/favicon/{}", ip.clone()));
            e.field(format!("Spieler:"), format!("{}/{}", status.players.online, status.players.max), false);
            e.field(format!("Ping:"), format!("{}ms", latency), false);
            e
        })
    }).await?;
    Ok(())
}

#[command]
async fn uwu(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let output: String = uwuify(args.rest());

    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e|{
            e.title("UwU");
            e.description(output);
            e.author(|a|{
                a.name(&msg.author.name);
                a.icon_url(&msg.author.avatar_url().unwrap());
                a
            })
        })
    }).await?;

    Ok(())
}

#[command]
async fn panzer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e|{
            e.title("SCHWERER PANZERSPÄHWAGEN SIEBEN KOMMA FÜNF ZENTIMETER SONDERKRAFTFAHRZEUG ZWEIHUNDERTVIERUNDDREISSIG / VIER PANZERABWEHRKANONENWAGEN");
            e.description(args.rest());
            e.image("https://www.weltkrieg2.de/wp-content/uploads/2018/08/SdKfz-234-4-Munster01.jpg");
            e
        })
    }).await?;
    
    Ok(())
}

#[command]
async fn unsee(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e|{
            e.title(format!("{} Hat alles gesehen", &msg.author.name));
            e.description(args.rest());
            e.image("https://media1.tenor.com/images/eebda566750dca6978b373d920144259/tenor.gif");
            e
        })
    }).await?;

    Ok(())
}
