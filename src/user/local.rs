use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use log::error;
use thiserror::Error;

use crate::database::model::{NewUser, User};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Unknown Error")]
    Unknown,
}

pub fn create(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> QueryResult<User> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    diesel::insert_into(user)
    .values(&new_user)
    .execute(conn)
    /*.map_err(|e| DatabaseError::Unknown)*/
    .expect("Creation Error");
    user
    .filter(username.eq(new_user.username.to_owned()))
    .get_result(conn)
}