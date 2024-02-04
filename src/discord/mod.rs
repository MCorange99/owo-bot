pub mod event;
pub mod commands;

use serenity::{all::GatewayIntents, Client};
use anyhow::{bail, Result};

use crate::util::config::Config;

#[allow(dead_code)]
pub struct DiscordClient {
    client: Client
}

impl DiscordClient {
    pub async fn new(cfg: &Config) -> Result<Self> {

        let client = serenity::Client::builder(
            &cfg.discord.token, 
            GatewayIntents::all()
        )
            .raw_event_handler(event::Handler::new(&cfg))
            .await;

        let mut client = match client {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to create client");
                log::debug!("{e}");
                bail!("bad client")
            }
        };

        if let Err(e) = client.start().await {
            log::error!("Failed to start client");
            log::debug!("{e}");
            bail!("failed to log in")
        };

        Ok(Self {
            client
        })
    }

}