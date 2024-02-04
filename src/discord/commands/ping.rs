use clap::Parser;
use serenity::all::*;
use anyhow::Result;
use crate::util::config::Config;

#[derive(Debug, Parser)]
pub struct Args {}


pub async fn exec(_cfg: &Config, ctx: &Context, msg: &Message, _args: Args) -> Result<()> {
    msg.reply_ping(&ctx.http, "Owo?").await?;
    Ok(())   
}