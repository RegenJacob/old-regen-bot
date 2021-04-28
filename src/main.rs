mod discord;
mod fun;
mod jacob;
mod voice;
mod wiki;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    discord::start().await;
}
