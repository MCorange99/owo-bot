mod ready;
mod message;

use serenity::all::*;
use crate::{log_if_err, util::config::Config};

use super::{commands::CommandInfo, features::{autoresponder::Autoresponder, Feature, FeatureWraper}};

pub struct Handler {
    pub commands: Box<Vec<CommandInfo>>,
    pub config: Config,
    pub autoresponder: FeatureWraper<Autoresponder>
}


impl Handler {
    pub fn new(config: &Config) -> Self {
        Self {
            commands: Box::new(Vec::new()),
            config: config.clone(),
            autoresponder: FeatureWraper::new(Autoresponder::default()),
        }
    }
}

#[async_trait]
impl RawEventHandler for Handler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        match ev {
            Event::CommandPermissionsUpdate(_)      => (),
            Event::AutoModRuleCreate(_)             => (),
            Event::AutoModRuleUpdate(_)             => (),
            Event::AutoModRuleDelete(_)             => (),
            Event::AutoModActionExecution(_)        => (),
            Event::ChannelCreate(_)                 => (),
            Event::ChannelDelete(_)                 => (),
            Event::ChannelPinsUpdate(_)             => (),
            Event::ChannelUpdate(_)                 => (),
            Event::GuildAuditLogEntryCreate(_)      => (),
            Event::GuildBanAdd(_)                   => (),
            Event::GuildBanRemove(_)                => (),
            Event::GuildCreate(_)                   => (),
            Event::GuildDelete(_)                   => (),
            Event::GuildEmojisUpdate(_)             => (),
            Event::GuildIntegrationsUpdate(_)       => (),
            Event::GuildMemberAdd(_)                => (),
            Event::GuildMemberRemove(_)             => (),
            Event::GuildMemberUpdate(_)             => (),
            Event::GuildMembersChunk(_)             => (),
            Event::GuildRoleCreate(_)               => (),
            Event::GuildRoleDelete(_)               => (),
            Event::GuildRoleUpdate(_)               => (),
            Event::GuildStickersUpdate(_)           => (),
            Event::GuildUpdate(_)                   => (),
            Event::InviteCreate(_)                  => (),
            Event::InviteDelete(_)                  => (),
            Event::MessageDelete(_)                 => (),
            Event::MessageDeleteBulk(_)             => (),
            Event::MessageUpdate(_)                 => (),
            Event::PresenceUpdate(_)                => (),
            Event::PresencesReplace(_)              => (),
            Event::ReactionAdd(_)                   => (),
            Event::ReactionRemove(_)                => (),
            Event::ReactionRemoveAll(_)             => (),
            Event::ReactionRemoveEmoji(_)           => (),
            Event::Resumed(_)                       => (),
            Event::TypingStart(_)                   => (),
            Event::UserUpdate(_)                    => (),
            Event::VoiceStateUpdate(_)              => (),
            Event::VoiceServerUpdate(_)             => (),
            Event::VoiceChannelStatusUpdate(_)      => (),
            Event::WebhookUpdate(_)                 => (),
            Event::InteractionCreate(_)             => (),
            Event::IntegrationCreate(_)             => (),
            Event::IntegrationUpdate(_)             => (),
            Event::IntegrationDelete(_)             => (),
            Event::StageInstanceCreate(_)           => (),
            Event::StageInstanceUpdate(_)           => (),
            Event::StageInstanceDelete(_)           => (),
            Event::ThreadCreate(_)                  => (),
            Event::ThreadUpdate(_)                  => (),
            Event::ThreadDelete(_)                  => (),
            Event::ThreadListSync(_)                => (),
            Event::ThreadMemberUpdate(_)            => (),
            Event::ThreadMembersUpdate(_)           => (),
            Event::GuildScheduledEventCreate(_)     => (),
            Event::GuildScheduledEventUpdate(_)     => (),
            Event::GuildScheduledEventDelete(_)     => (),
            Event::GuildScheduledEventUserAdd(_)    => (),
            Event::GuildScheduledEventUserRemove(_) => (),
            
            Event::MessageCreate(e) => {
                {
                    log_if_err!(self.autoresponder.lock().await.run(&self.config, &ctx, &e.message).await);
                }

                self.message_create(ctx, &e.message).await;
            }
            Event::Ready(e) => {
                {
                    log_if_err!(self.autoresponder.lock().await.init(&self.config, &ctx, None).await);
                }
                self.ready(ctx, e.ready).await;
            }
            Event::Unknown(e)  => log::debug!("Unknown event: {:?} {:?}", e.kind, e.value),
            e => log::debug!("Very unknown event {e:?}")
        }
        
    }
    
}
