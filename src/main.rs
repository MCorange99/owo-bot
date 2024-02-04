use dotenvy;

mod util;
mod discord;


macro_rules! check_res {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => {
                log::debug!("{e}");
                return;
            }
        }
    };
}

#[tokio::main]
async fn main() {
    if let Err(e) = dotenvy::dotenv() {
        println!("[early] ERROR: Unable to load ENV vars");
        println!("[early] ERROR: {e}");
        return;
    }
    env_logger::init();

    let mut cfg = check_res!(
        util::config::Config::parse()
    );

    let _client = check_res!(
        discord::DiscordClient::new(&mut cfg).await
    );
}
