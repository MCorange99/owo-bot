use anyhow::{bail, Result};
use diesel::{Connection, PgConnection};

use crate::util::config::Config;

pub mod schema;
pub mod models;
pub mod actions;

pub struct Database {
    pub connection: PgConnection   
}


impl Database {
    pub fn connect(cfg: &Config) -> Result<Self> {
        let conn = match PgConnection::establish(&cfg.main.database.get_db_url()) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to connect to database: {e}");
                bail!("Failed to connect")
            },
        };

        log::info!("Connected to {} database", cfg.main.database.get_db_url_censored());

        Ok(Self {
            connection: conn
        })
    }
}