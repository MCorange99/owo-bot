use dotenvy;

use crate::database::Database;

mod util;
mod discord;
mod database;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv(); // We dont care if it loads
    env_logger::init();
    let mut cfg = util::config::Config::new();
    check_res!(cfg.parse());

    let db = check_res!(Database::connect(&cfg));

    let _client = check_res!(
        discord::DiscordClient::new(&mut cfg).await
    );
}
