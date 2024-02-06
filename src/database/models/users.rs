use diesel::prelude::*;
use crate::database::schema;
#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub uid: i32,
    pub groups: Option<Vec<Option<i32>>>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub uid: i32,
}