use diesel::prelude::*;

use crate::database::Database;
use crate::database::{schema, models};

pub fn create_user(db: &mut Database, uid: i32) -> anyhow::Result<models::users::User> {

    let new_user = models::users::NewUser { uid };

    let res = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .returning(models::users::User::as_returning())
        .get_result(&mut db.connection);

    match res {
        Ok(u) => return Ok(u),
        Err(e) => {
            log::error!("Failed to create new user: {e}");
            anyhow::bail!("db insert failed")
        },
    }
}

pub fn get_user(db: &mut Database, user_id: i32) -> Option<models::users::User> {
    use crate::database::schema::users::dsl::*;
    let res = users
        .filter(uid.eq(user_id))
        .select(models::users::User::as_select())
        .first(&mut db.connection)
        .optional();

    match res {
        Ok(u) => return u,
        Err(e) => {
            log::error!("Had an error while trying to get user: {e}");
            return None;
        },
    }
}

pub fn set_user_groups(db: &mut Database, user_id: i32, _groups: Vec<i32>) -> anyhow::Result<models::users::User> {
    use crate::database::schema::users::dsl::*;
    let res = diesel::update(users.find(user_id))
        .set(groups.eq(_groups))
        .returning(models::users::User::as_returning())
        .get_result(&mut db.connection);

    match res {
        Ok(u) => Ok(u),
        Err(e) => {
            log::error!("Had an error while trying to get user: {e}");
            anyhow::bail!("db edit failed")
        },
    }
}