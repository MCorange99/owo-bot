use std::num::NonZeroU16;

use serenity::all::*;
use super::Handler;

impl Handler {
    pub async fn ready(&self, _: Context, ready: Ready) {
        log::info!("Logged in as {}#{} ({})", 
            ready.user.name,
            ready.user.discriminator.unwrap_or(NonZeroU16::new(1).unwrap()),
            ready.user.id
        );
    }
}