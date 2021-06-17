use crate::discord::check_msg;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use std::time::Duration;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use uwuifier::uwuify_str_sse;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn uwuify(input: &str) -> String {
    uwuify_str_sse(input)
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
fn uwuify(input: &str) -> String {
    "Not implemented! :("
}

#[group]
#[commands(mcstatus, uwu, panzer, unsee, joke, mcname, meme)]
pub struct Fun;

#[command]
async fn meme(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let number = match args.single::<i32>() {
        Ok(number) => number,
        Err(_) => 1 as i32
    };

    if number < 20 {
        msg.reply(&ctx.http, "number Can't be higher then 20!").await?;
        return Ok(());
    }

    while number >= 0 {
        msg.reply(&ctx.http, "Hi").await?;
    } 

    Ok(())
}

#[command]
async fn mcstatus(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let ip = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Server IP ist ungültig")
                    .await,
            );

            return Ok(());
        }
    };

    let a = ip.clone();

    let (latency, status) = tokio::task::block_in_place(move || {
        let (latency, status) =
            mcping::get_status(&*a, Duration::from_secs(10)).expect("Not a Server");
        (latency, status)
    });

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Info über {}", ip.clone()));
                e.description(status.description.text());
                e.thumbnail(format!(
                    "https://eu.mc-api.net/v3/server/favicon/{}",
                    ip.clone()
                ));
                e.field(
                    "Spieler:",
                    format!("{}/{}", status.players.online, status.players.max),
                    false,
                );
                e.field("Ping:", format!("{}ms", latency), false);
                e
            })
        })
        .await?;
    Ok(())
}

#[command]
async fn mcname(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let username = args.single::<String>().unwrap();

    let client = reqwest::Client::new()
        .get(format!(
            "https://some-random-api.ml/mc?username={}",
            username
        ))
        //.header("username", "RegenJacob")
        .send()
        .await?;

    if client.status().as_u16() != 200 {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("{} exestiert nicht!", username));
                    e.description("oder ein anderer Fehler ist aufgetreten");
                    e.footer(|f| f.text(format!("{} error code", client.status())))
                })
            })
            .await?;
    }

    let body = client.text().await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Infos über {}", username));
                e.thumbnail(format!("https://minotar.net/helm/{}/100.png", username));
                e.description(body)
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn uwu(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let output: String = uwuify(args.rest());

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("UwU");
                e.description(output);
                e.author(|a| {
                    a.name(&msg.author.name);
                    a.icon_url(&msg.author.avatar_url().unwrap());
                    a
                })
            })
        })
        .await?;

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
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{} Hat alles gesehen", &msg.author.name));
                e.description(args.rest());
                e.image(
                    "https://media1.tenor.com/images/eebda566750dca6978b373d920144259/tenor.gif",
                );
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let resp = reqwest::get("https://v2.jokeapi.dev/joke/Any?lang=en&blacklistFlags=nsfw,religious,political,racist,sexist")
        .await?
        .text()
        .await?;

    let res = serde_json::from_str(&*resp);

    if res.is_ok() {
        let p: serde_json::Value = res.unwrap();

        if p["type"] == "twopart" {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(format!("{}", p["setup"]));
                        e.description(format!("||{}||", p["delivery"].as_str().unwrap()));
                        e.footer(|f| {
                            f.text("source https://sv443.net/jokeapi/v2/");
                            f.icon_url("https://sv443.net/jokeapi/v2/favicon.ico");
                            f
                        });
                        e
                    })
                })
                .await?;
        } else {
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(p["joke"].as_str().unwrap());
                        e.footer(|f| {
                            f.text("source https://sv443.net/jokeapi/v2/");
                            f.icon_url("https://sv443.net/jokeapi/v2/favicon.ico");
                            f
                        });
                        e
                    })
                })
                .await?;
        }
    }

    Ok(())
}

#[command]
async fn user(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.remains() == None {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!("Info über {}", msg.author.name));

                    e
                })
            })
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| m.content("Not implementet yet!"))
            .await?;
    }
    Ok(())
}
