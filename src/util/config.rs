use serde::Deserialize;
use anyhow::{bail, Result};
use serenity::all::{EmojiId, ReactionType};

const DEF_CFG: &'static str = include_str!("../../default.config.toml");
const CFG_PTH: &'static str = "./config.toml";


#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub discord: ConfigDiscord,
}

#[derive(Deserialize, Clone, Default)]
pub struct ConfigDiscord {
    pub token: String,
    pub prefix: String,
    pub admin_role: u64,
    pub mod_role: u64,
    pub autoreply: Vec<ConfigAutoreply>,
    pub autoreact: Vec<ConfigAutoreact>
}

#[derive(Deserialize, Clone)]
pub struct ConfigAutoreply {
    pub trigger: String,
    pub reply: String
}

#[derive(Deserialize, Clone)]
pub struct ConfigAutoreact {
    pub trigger: Vec<String>,
    pub name: Option<String>,
    pub id: String,
    pub animated: bool
}


impl Config {
    pub fn parse() -> Result<Self> {

        let s = match std::fs::read_to_string("./config.toml") {
            Ok(s) => s,
            Err(_) => {
                if let Err(e) = std::fs::write(CFG_PTH, DEF_CFG) {
                    log::error!("Failed to write default configs to new file");
                    log::debug!("{e}");
                }
                log::error!("No config file found, making one, please populate it");
                bail!("New cfg made");
            },
        };

        match toml::from_str::<Self>(&s) {
            Ok(r) => Ok(r),
            Err(e) => {
                log::error!("Failed to parse config file");
                log::debug!("{e}");
                bail!("bad cfg file");
            },
        }
    }
}

impl ConfigAutoreact {
    pub fn to_custom_reaction(&self) -> ReactionType {
        ReactionType::Custom { animated: self.animated, id: EmojiId::new(self.id.parse().expect("Invalid emoji id")), name: self.name.clone() }
    }
}