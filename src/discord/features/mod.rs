pub mod autoresponder;
pub mod on_reload;

pub use anyhow::Result;
use async_mutex::{Mutex, MutexGuard};
use serenity::client::Context;
use crate::util::config::Config;
pub trait Feature {
    type Extra;

    async fn init(&mut self, cfg: &Config, ctx: &Context, ex: Option<&Self::Extra>) -> Result<()>;
    async fn run(&mut self, cfg: &Config, ctx: &Context, ex: &Self::Extra) -> Result<()>;
    async fn stop(&mut self, cfg: &Config, ctx: &Context, ex: Option<&Self::Extra>) -> Result<()>;
}

pub struct FeatureWraper<T>(Mutex<T>);

impl<T> FeatureWraper<T> where T: Default {
    pub fn new(v: T) -> Self {
        FeatureWraper(Mutex::new(v))
    }
    
    pub async fn lock(&self) -> MutexGuard<T>{       
        self.0.lock().await
    }
}