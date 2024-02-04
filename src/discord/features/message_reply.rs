use serenity::all::*;

use crate::{msg_reply, util::config::Config};

pub async fn autoreply(cfg: &Config, ctx: &Context, msg: &Message) {
    
    for s in &cfg.discord.autoreply {
        if msg.content.to_ascii_lowercase().contains(&s.trigger) {
            msg_reply!(msg, ctx, "{}", s.reply);
        }
    }
}

pub async fn autoreact(cfg: &Config, ctx: &Context, msg: &Message) {   
    for ar in &cfg.discord.autoreact {
        for trig in &ar.trigger {
            if msg.content.to_ascii_lowercase().contains(trig) {
                if let Err(e) = msg.react(&ctx.http, ar.to_custom_reaction()).await {
                    log::error!("Failed to react message ({}) with <:{}:{}> : {e}", msg.id, ar.name.clone().unwrap_or(String::new()), ar.id)
                }
            }
        }
    }
}