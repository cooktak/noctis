use diesel::{MysqlConnection, result::Error as DieselError};

use crate::database::model::{Device, User};
use crate::device::DeviceError;

pub fn validate(
    conn: &MysqlConnection,
    query_token: String,
) -> Result<Device, DeviceError> {
    let query: (Device, User) = {
        use crate::database::schema::{device::dsl::*, user};
        use diesel::prelude::*;

        device
        .filter(token.eq(query_token))
        .inner_join(user::table)
        .get_result(conn)
        .map_err(|e| {
            match e {
                DieselError::NotFound => DeviceError::NotFound,
                _ => DeviceError::Unknown(e.to_string()),
            }
        })
    }?;

    Ok(query.0)
}