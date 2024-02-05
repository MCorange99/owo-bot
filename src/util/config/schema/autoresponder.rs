use serde::Deserialize;
use serenity::all::{EmojiId, ReactionType};

#[derive(Deserialize, Clone, Default)]
pub struct ConfigAutoresponder {
    pub reply: Vec<ConfigAutoreply>,
    pub react: Vec<ConfigAutoreact>,
}

#[derive(Deserialize, Clone, Default)]
pub struct ConfigAutoreply {
    pub trigger: Option<String>,
    pub trigger_rx: Option<String>,
    pub response: String,
    pub force_lowercase: Option<bool>
}

#[derive(Deserialize, Clone, Default)]
pub struct ConfigAutoreact {
    pub trigger: Option<String>,
    pub trigger_rx: Option<String>,
    pub name: Option<String>,
    pub id: String,
    pub animated: bool,
    pub force_lowercase: Option<bool>,
}

impl ConfigAutoreact {
    pub fn to_custom_reaction(&self) -> ReactionType {
        ReactionType::Custom { animated: self.animated, id: EmojiId::new(self.id.parse().expect("Invalid emoji id")), name: self.name.clone() }
    }
}