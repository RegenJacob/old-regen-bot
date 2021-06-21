mod discord;
mod commands;
mod web_interface;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        web_interface::start().unwrap();
    });
    discord::start().await;
}
