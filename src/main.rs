mod discord;
mod fun;
mod owner;
mod voice;
mod wiki;

#[tokio::main]
async fn main() {
    println!("Starting Bot!");
    discord::start().await;
}
