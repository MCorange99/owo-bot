use dotenvy;

mod util;
mod discord;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv(); // We dont care if it loads
    env_logger::init();
    let mut cfg = util::config::Config::new();
    check_res!(cfg.parse());

    let _client = check_res!(
        discord::DiscordClient::new(&mut cfg).await
    );
}
