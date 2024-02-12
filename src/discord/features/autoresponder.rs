use regex::Regex;
use serenity::all::*;
use anyhow::Result;
use crate::{msg_reply, util::config::Config};
use std::collections::HashMap;
use super::Feature;

const RL_TIMEOUT: usize = 5000;

#[derive(Debug, Default)]
pub struct Autoresponder {
    ratelimit: HashMap<UserId, Timestamp>
}

impl Autoresponder {

    ///
    async fn has_ratelimit(&mut self, usr: UserId) -> bool {
        if let Some(t) = self.ratelimit.get(&usr) {
            let td = Timestamp::now().timestamp_millis() - t.timestamp_millis();
            if (td as usize) < RL_TIMEOUT {
                true
            } else {
                self.ratelimit.insert(usr, Timestamp::now());
                false
            }
        } else {
            self.ratelimit.insert(usr, Timestamp::now());
            false
        }
    }
}


impl Feature for Autoresponder {
    type Extra = Message;

    async fn init(&mut self, _: &Config, _: &Context, _: Option<&Self::Extra>) -> Result<()>{
        Ok(())
    }
    async fn stop(&mut self, _: &Config, _: &Context, _: Option<&Self::Extra>) -> Result<()>{
        Ok(())
    }

    //cfg: &Config, ctx: &Context, msg: &Message
    async fn run(&mut self, cfg: &Config, ctx: &Context, msg: &Self::Extra) -> Result<()>{
        
        if self.has_ratelimit(msg.author.id).await {
            log::warn!("Ratelimited user {}({})", msg.author.name, msg.author.id);
            return Ok(())
        }



        for ar in &cfg.autoresonder.reply {
            let txt = if ar.keep_case == Some(false) || ar.keep_case == None{
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

        
        for ar in &cfg.autoresonder.react {
            let txt = if ar.keep_case == Some(false) || ar.keep_case == None{
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

        Ok(())
    }
}