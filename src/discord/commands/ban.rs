
use clap::Parser;
// use humantime::Duration;
use serenity::all::*;
use anyhow::Result;
use crate::{msg_reply, util::config::Config};

use super::{CommandInfo, CommandPerms};

pub const INFO: CommandInfo = CommandInfo {
    name: "ban",
    usage: "ban -u @somecunt -r 'He was being dumb' -d 7",
    permissions: CommandPerms::MOD,
};

///
/// Ban users for a specified amount of time
/// 
#[derive(Debug, Parser)]
pub struct Args {

    /// The user to ban
    #[arg(long, short)]
    user: String,

    /// For how long, if left blank, permenantely
    #[arg(long, short)]
    time: Option<String>,

    /// Why you want to ban this user
    #[arg(long, short)]
    reason: Option<String>,

    /// How many days worth of messages to remove made by the user, up to 7 days
    #[arg[long, short, default_value_t=0]]
    delete: u8
}


pub async fn exec(_cfg: &Config, ctx: &Context, msg: &Message, args: Args) -> Result<()> {
    let user_id = serenity::utils::parse_user_mention(&args.user);
    let user = match user_id {
        Some(id) => ctx.http.get_user(id).await,
        None => {
            let Ok(id) = args.user.parse() else {
                msg_reply!(msg, ctx, "Unknown user {}", args.user);
                return Ok(());
            };
            ctx.http.get_user(UserId::new(id)).await
        }
    };
    let Ok(mut user) = user else {
        msg_reply!(msg, ctx, "Unknown user {}", args.user);
        return Ok(());
    };


    user.refresh(&ctx.http).await?;


    let guild = msg.guild(&ctx.cache).unwrap().clone();

    let res = match args.reason {
        Some(r) => guild.ban_with_reason(&ctx.http, user.clone(), args.delete, r).await,
        None => guild.ban(&ctx.http, user.clone(), args.delete).await,
    };

    match res {
        Ok(_) => msg_reply!(msg, ctx, "Banned user {} ({})", user.name, user.id),
        Err(e) => msg_reply!(msg, ctx, "Couldnt ban user {} ({}) reason: {e}", user.name, user.id),
    };
    Ok(())
}

