pub mod discord;
pub mod autoresponder;
pub use autoresponder::*;

use serde::Deserialize;
use self::discord::ConfigDiscord;

#[derive(Deserialize, Clone, Default)]
pub struct ConfigMain {
    pub discord: ConfigDiscord,
}