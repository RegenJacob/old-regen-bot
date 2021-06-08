use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[group]
#[commands(wikisearch)]
pub struct Wiki;

#[command]
async fn wikisearch(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let results = wiki.search(args.rest()).unwrap();
    if results.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Kein Wikipedia Eintrag Gefunden!");
                    e.description(format!("{} wurde nicht gefunden!", args.rest()));
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let page = wiki.page_from_title(results[0].to_string());

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&page.get_title().unwrap());
            e.author(|a| {
                a.name("Wikipedia");
                a.icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Wikipedia-logo-v2.svg/150px-Wikipedia-logo-v2.svg.png");
                a.url("https://wikipedia.org/")
            });
            e.description(&page.get_summary().unwrap());
            e.footer(|f| {
                f.text("https://wikipedia.org/");
                f.icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/8/80/Wikipedia-logo-v2.svg/150px-Wikipedia-logo-v2.svg.png")
            });
            e

        })
    }).await?;
    Ok(())
}
