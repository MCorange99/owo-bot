use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use diesel::{r2d2::{ConnectionManager, Pool}, Connection, PgConnection};
use serenity::prelude::TypeMapKey;

use crate::util::config::Config;

pub mod schema;
pub mod models;
pub mod actions;

#[derive(Clone)]
pub struct Database  {
    pub connection: Pool<ConnectionManager<PgConnection>>
}

pub struct DatabaseContainer {}

impl TypeMapKey for DatabaseContainer {
    type Value = Database;
}


impl Database {
    pub fn connect(cfg: &Config) -> Result<Self> {

        let conn = Pool::new(ConnectionManager::<PgConnection>::new(&cfg.main.database.get_db_url()));

        let conn = match conn {
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