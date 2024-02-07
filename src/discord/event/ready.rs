use std::num::NonZeroU16;

use serenity::all::*;
use crate::discord::{commands::register_commands, features::on_reload::on_reload};

use super::Handler;

impl Handler {
    pub async fn ready(&self, ctx: Context, ready: Ready) {
        on_reload(ctx)
        register_commands();
        log::info!("Logged in as {}#{} ({}) with shard ID #{}", 
            ready.user.name,
            ready.user.discriminator.unwrap_or(NonZeroU16::new(1).unwrap()),
            ready.user.id,
            ctx.shard_id
        );
    }
}