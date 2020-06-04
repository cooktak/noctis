use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use diesel::result::Error as DieselError;
use log::error;
use thiserror::Error;

use crate::database::model::{NewUser, User};

use super::error::UserError;

pub fn create(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> Result<User, UserError> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;

    let query = user.filter(username.eq(&new_user.username));
    if let Ok(Some(_)) = query.get_result::<User>(conn).optional() {
        return Err(UserError::Exists);
    }

    diesel::insert_into(user)
    .values(&new_user.to_hashed())
    .execute(conn)
    .map_err(|e| {
        match e {
            _ => UserError::Unknown(e.to_string()),
        }
    })?;

    user
    .filter(username.eq(&new_user.username))
    .filter(birthday.eq(new_user.birthday))
    .get_result(conn)
    .map_err(|e| {
        match e {
            _ => UserError::Unknown(e.to_string()),
        }
    })
}

pub fn query(
    conn: &MysqlConnection,
    query_user_name: &String,
    query_user_tag: i32,
) -> Result<User, UserError> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    user
    .filter(username.eq(query_user_name))
    .filter(user_tag.eq(query_user_tag))
    .get_result(conn)
    .map_err(|e| {
        match e {
            DieselError::NotFound => UserError::NotFound(query_user_name.clone()),
            _ => UserError::Unknown(e.to_string()),
        }
    })
}
