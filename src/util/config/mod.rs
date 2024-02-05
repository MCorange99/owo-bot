mod schema;

use maplit::hashmap;
use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Deserialize;
use anyhow::{bail, Result};


lazy_static!(
    static ref DEF_CFGS: HashMap<&'static str, &'static str> = hashmap! {
        "main"          => include_str!("../../../config/default.main.toml"),
        "autoresponder" => include_str!("../../../config/default.autoresponder.toml"),
    };
);

#[derive(Deserialize, Clone, Default)]
pub struct Config {
    pub main: schema::ConfigMain,
    pub autoresonder: schema::ConfigAutoresponder,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&mut self) -> Result<&mut Self> {
        self.main         = self.load_cfg::<schema::ConfigMain>("main")?;
        self.autoresonder = self.load_cfg::<schema::ConfigAutoresponder>("autoresponder")?;
        Ok(self)
    }

    pub fn load_cfg<T: for<'a> Deserialize<'a>>(&mut self, name: &str) -> Result<T> {
        let s = match std::fs::read_to_string(format!("./config/{name}.toml")) {
            Ok(s) => s,
            Err(_) => {
                if let Err(e) = std::fs::write(format!("./config/{name}.toml"), DEF_CFGS.get(name).unwrap()) {
                    log::error!("Failed to write default configs to new file ({name})");
                    log::debug!("{e}");
                }
                log::error!("No config {name} file found, making one, please populate it");
                bail!("New cfg made");
            },
        };

        match toml::from_str::<T>(&s) {
            Ok(r) => Ok(r),
            Err(e) => {
                log::error!("Failed to parse config file");
                log::debug!("{e}");
                bail!("bad cfg file");
            },
        }
    }
}