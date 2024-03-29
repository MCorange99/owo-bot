use serenity::all::*;
use super::Handler;

impl Handler {
    pub async fn message_create(&self, ctx: Context, msg: &Message) {
        if msg.is_own(&ctx.cache) {
            return; // Dont respond to own messages
        }

        if msg.kind != MessageType::Regular &&
            msg.kind != MessageType::InlineReply {
            return; // Only reply to normal messages and replys
        }

        if msg.content.starts_with(&self.config.main.discord.prefix) {
            let Some(txt) = msg.content.strip_prefix(&self.config.main.discord.prefix) else {
                log::debug!("Failed to strip prefix from message {:?}", msg.content);
                return;
            };

            let args = match shellish_parse::parse(txt, true) {
                Ok(r) => r,
                Err(e) => {
                    let _ = msg.reply_ping(&ctx.http, format!("Failed to parse command {txt:?}: {e}")).await;
                    return;
                }
            };


            match crate::discord::commands::command_handler(&self.config, &self.commands, &ctx, msg, args).await {
                Ok(_) => (),
                Err(e) => {
                    let _ = msg.reply_ping(&ctx.http, format!("{e}")).await;
                }
            }

        }
    }
}