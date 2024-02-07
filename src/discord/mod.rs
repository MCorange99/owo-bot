pub mod event;
pub mod commands;
pub mod features;

use serenity::{all::GatewayIntents, Client};
use anyhow::{bail, Result};

use crate::{database::{Database, DatabaseContainer}, util::config::Config};

#[allow(dead_code)]
pub struct DiscordClient {
    client: Client
}

impl DiscordClient {
    pub async fn new(db: &mut Database, cfg: &Config) -> Result<Self> {
        log::info!("Connecting to discord");
        let client = serenity::Client::builder(
            &cfg.main.discord.token, 
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

        client.data.write().await.insert::<DatabaseContainer>(db.clone());
        

        if let Err(e) = client.start_autosharded().await {
            log::error!("Failed to start client");
            log::debug!("{e}");
            bail!("failed to log in")
        };

        Ok(Self {
            client
        })
    }

}