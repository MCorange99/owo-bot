use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct ConfigDiscord {
    pub token: String,
    pub prefix: String,
    pub admin_role: u64,
    pub mod_role: u64,
}