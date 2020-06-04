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
    .values(&new_user.to_hashed())
    .execute(conn)
    .expect("Creation Error");
    user
    .filter(username.eq(new_user.username.to_owned()))
    .filter(birthday.eq(new_user.birthday))
    .get_result(conn)
}

pub fn query(
    conn: &MysqlConnection,
    query_user_name: &String,
    query_user_tag: i32,
) -> QueryResult<User> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    user
    .filter(username.eq(query_user_name))
    .filter(user_tag.eq(query_user_tag))
    .get_result(conn)
}
