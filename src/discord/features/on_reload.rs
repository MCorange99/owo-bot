use std::borrow::BorrowMut;

use serenity::client::Context;

use crate::database::{self, DatabaseContainer};



pub async fn on_reload(ctx: &Context) -> anyhow::Result<()> {
    
    {
        // ctx.data.write().await.entry::<DatabaseContainer>().and_modify(|db| {
        //     database::actions::users::create_user(&mut db, uid)?;
        // });
    }
    Ok(())
}