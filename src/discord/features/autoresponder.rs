use regex::Regex;
use serenity::all::*;

use crate::{msg_reply, util::config::Config};

pub async fn autoreply(cfg: &Config, ctx: &Context, msg: &Message) {

    for ar in &cfg.autoresonder.reply {
        let txt = if ar.force_lowercase == Some(false) {
            msg.content.to_ascii_lowercase()
        } else {
            msg.content.clone()
        };
        let txt = txt.as_str();

        if let Some(trigger) = &ar.trigger {
            if txt.contains(trigger) {
                msg_reply!(msg, ctx, "{}", ar.response);
            }
        }
        if let Some(trigger) = &ar.trigger_rx {
            let Ok(rx) = Regex::new(&trigger) else {
                log::error!("Invalid regex {trigger:?}");
                continue;
            };

            if rx.is_match(txt) {
                msg_reply!(msg, ctx, "{}", ar.response);
            }
        }
    }
}

pub async fn autoreact(cfg: &Config, ctx: &Context, msg: &Message) {   
    for ar in &cfg.autoresonder.react {
        let txt = if ar.force_lowercase == Some(false) {
            msg.content.to_ascii_lowercase()
        } else {
            msg.content.clone()
        };
        let txt = txt.as_str();

        if let Some(trigger) = &ar.trigger {
            if txt.contains(trigger) {
                if let Err(e) = msg.react(&ctx.http, ar.to_custom_reaction()).await {
                    log::error!("Failed to react message ({}) with <:{}:{}> : {e}", msg.id, ar.name.clone().unwrap_or(String::new()), ar.id)
                }
            }
        }
        if let Some(trigger) = &ar.trigger_rx {
            let Ok(rx) = Regex::new(&trigger) else {
                log::error!("Invalid regex {trigger:?}");
                continue;
            };

            if rx.is_match(txt) {
                if let Err(e) = msg.react(&ctx.http, ar.to_custom_reaction()).await {
                    log::error!("Failed to react message ({}) with <:{}:{}> : {e}", msg.id, ar.name.clone().unwrap_or(String::new()), ar.id)
                }
            }
        }
    }

}