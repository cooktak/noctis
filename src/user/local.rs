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
    .values(&new_user.to_hashed())
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
        match e {
            _ => UserError::Unknown(e.to_string()),
        }
    })
}

pub fn query(
    conn: &MysqlConnection,
    query_user_name: &String,
) -> Result<User, UserError> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    user
    .filter(username.eq(query_user_name))
    .get_result(conn)
    .map_err(|e| {
        match e {
            DieselError::NotFound => UserError::NotFound(query_user_name.clone()),
            _ => UserError::Unknown(e.to_string()),
        }
    })
}

pub fn authentication(
    conn: &MysqlConnection,
    auth_user_name: &String,
    auth_password: &String,
    auth_device_name: &String,
) -> Result<Device, UserError> {
    let user = query(conn, auth_user_name)?;
    let hashed_password = User::hashed_password(auth_password, auth_user_name);
    if !user.password.eq(&hashed_password) {
        return Err(UserError::Authentication);
    }

    device::register(conn, user, auth_device_name.clone())
}
