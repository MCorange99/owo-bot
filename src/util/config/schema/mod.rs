pub mod discord;
pub mod autoresponder;
pub mod database;
pub use autoresponder::*;

use serde::Deserialize;
use self::{database::ConfigDatabase, discord::ConfigDiscord};

#[derive(Deserialize, Clone, Default)]
pub struct ConfigMain {
    pub discord: ConfigDiscord,
    pub database: ConfigDatabase
}