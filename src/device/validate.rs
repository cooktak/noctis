use diesel::{MysqlConnection, result::Error as DieselError};

use crate::database::model::Device;
use crate::device::DeviceError;

pub fn validate(
    conn: &MysqlConnection,
    query_token: String,
) -> Result<Device, DeviceError> {
    use crate::database::schema::device::dsl::*;
    use diesel::prelude::*;
    device
    .filter(token.eq(query_token))
    .get_result(conn)
    .map_err(|e| {
        match e {
            DieselError::NotFound => DeviceError::NotFound,
            _ => DeviceError::Unknown(e.to_string()),
        }
    })
}