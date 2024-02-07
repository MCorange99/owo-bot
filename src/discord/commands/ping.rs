use clap::Parser;
use serenity::all::*;
use anyhow::Result;
use crate::util::config::Config;

use super::{CommandInfo, CommandPerms};

pub const INFO: CommandInfo = CommandInfo {
    name: "ping",
    usage: "ping",
    permissions: CommandPerms::DEFAULT,
};


#[derive(Debug, Parser)]
pub struct Args {}


pub async fn exec(_cfg: &Config, ctx: &Context, msg: &Message, _args: Args) -> Result<()> {
    // ctx.shard.send_to_shard(ShardRunnerMessage::Restart(ShardId(0)))
    msg.reply_ping(&ctx.http, "Pong!").await?;
    Ok(())   
}