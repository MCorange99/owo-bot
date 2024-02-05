use std::{default, sync::RwLock};

use bitflags::bitflags;
use lazy_static::lazy_static;
use serenity::{all::Message, client::Context};
use anyhow::{bail, Result};
use crate::{add_cmd, msg_reply, run_cmd, util::config::Config};
use clap::Parser;

use super::event::Handler;

mod ping;
mod ban;

lazy_static!(
    pub static ref COMMANDS: RwLock<Vec<CommandInfo>> = {
        RwLock::new(Vec::new())
    };
);


bitflags! {    
    #[derive(Debug)]
    pub struct CommandPerms: u8 {
        const DEFAULT = 0;
        const MOD     = 1;
        const ADMIN   = 2;
    }
}



pub struct CommandInfo {
    name: &'static str,
    usage: &'static str,
    permissions: CommandPerms
}




pub fn register_commands(){
    *COMMANDS.write().unwrap() = vec![
        ping::INFO,
        ban::INFO,
    ];
}

pub async fn command_handler(cfg: &Config, commands: &Box<Vec<CommandInfo>>, ctx: &Context, msg: &Message, args: Vec<String>) -> Result<()> {

    
    check_perms(cfg, commands, ctx, msg, args[0].as_str()).await;

    
    let res = match args[0].as_str() {
        "ping" => run_cmd!(&cfg, ctx, msg, args, ping),
        "ban"  => run_cmd!(&cfg, ctx, msg, args, ban),
        _ => bail!("Unknown command {}", args.join(" ").escape_debug())
    };

    if let Err(e) = res {
        bail!("Command exited with error: {e}");
    }

    Ok(())
}

async fn check_perms(cfg: &Config, commands: &Box<Vec<CommandInfo>>, ctx: &Context, msg: &Message, cmd: &str) -> bool {

    let Some(cmd) = commands.iter().find(|c| c.name == cmd.to_string()) else {return false};
    let perms = &cmd.permissions;

    let perm_names = perms.iter_names().map(|f| f.0.to_string()).collect::<Vec<String>>().join(" ");

    let has_admin_role = msg.author.has_role(&ctx.http, msg.guild_id.unwrap(), cfg.main.discord.admin_role).await.unwrap_or(false);
    let has_mod_role = msg.author.has_role(&ctx.http, msg.guild_id.unwrap(), cfg.main.discord.mod_role).await.unwrap_or(false);

    match perms {
        p if p.contains(CommandPerms::ADMIN) && has_admin_role => true,
        p if p.contains(CommandPerms::MOD) && has_mod_role => true,
        _ => {
            msg_reply!(msg, ctx, "This command requires {}", perm_names);
            false
        },
    }



}