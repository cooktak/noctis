use std::str;

use diesel::MysqlConnection;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::database::model::{Device, NewUser, User};
use crate::device;

use super::error::UserError;

pub fn create(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> Result<User, UserError> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;

    diesel::insert_into(user)
    .values(&(match new_user.to_hashed() {
        Ok(u) => Ok(u),
        Err(e) => Err(UserError::Unknown(e.to_string()))
    }?))
    .execute(conn)
    .map_err(|e| {
        match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => UserError::Exists,
            _ => UserError::Unknown(e.to_string()),
        }
    })?;

    user
    .filter(username.eq(&new_user.username))
    .get_result(conn)
    .map_err(|e| {
        UserError::Unknown(e.to_string())
    })
}

pub fn query(
    conn: &MysqlConnection,
    query_user_name: &str,
) -> Result<User, UserError> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    user
    .filter(username.eq(query_user_name))
    .get_result(conn)
    .map_err(|e| {
        match e {
            DieselError::NotFound => UserError::NotFound(String::from(query_user_name)),
            _ => UserError::Unknown(e.to_string()),
        }
    })
}

pub fn authentication(
    conn: &MysqlConnection,
    auth_user_name: &str,
    auth_password: &str,
    auth_device_name: &str,
) -> Result<Device, UserError> {
    let user = query(conn, auth_user_name)?;
    let hashed_password = match User::hashed_password(&Vec::from(auth_password), auth_user_name) {
        Ok(p) => Ok(p),
        Err(e) => Err(UserError::Unknown(e.to_string()))
    }?;
    if !user.password.eq(&hashed_password) {
        return Err(UserError::Authentication);
    }

    device::register(conn, user, String::from(auth_device_name))
}
