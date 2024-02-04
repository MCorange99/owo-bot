use serenity::{all::Message, client::Context};
use anyhow::{bail, Result};
use crate::util::config::Config;
use clap::Parser;

mod ping;


macro_rules! run_cmd {
    ($cfg:expr, $ctx:expr, $msg:expr, $args:expr, $p:tt) => {
        match $p::Args::try_parse_from($args) {
            Ok(args) => $p::exec($cfg, $ctx, $msg, args).await,
            Err(e) => {
                let _ = $msg.reply_ping($ctx.http.clone(), format!("```sh\n{}\n```", e)).await;
                Ok(())
            }
        }
    };
}

pub async fn command_handler(cfg: &Config, ctx: &Context, msg: &Message, args: Vec<String>) -> Result<()> {
    let res = match args[0].as_str() {
        "ping" => run_cmd!(cfg, ctx, msg, args, ping),

        _ => bail!("Unknown command {}", args.join(" ").escape_debug())
    };

    if let Err(e) = res {
        bail!("Command exited with error: {e}");
    }

    Ok(())
}