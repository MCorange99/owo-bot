use serde::Deserialize;
use anyhow::{bail, Result};

const DEF_CFG: &'static str = include_str!("../../default.config.toml");
const CFG_PTH: &'static str = "./config.toml";


#[derive(Deserialize, Clone)]
pub struct Config {
    pub discord: ConfigDiscord
}

#[derive(Deserialize, Clone)]
pub struct ConfigDiscord {
    pub token: String,
    pub prefix: String,  
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