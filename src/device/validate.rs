use diesel::{MysqlConnection, result::Error as DieselError};

use crate::database::model::{Device, User};
use crate::device::DeviceError;

pub fn validate(
    conn: &MysqlConnection,
    query_token: String,
) -> Result<Device, DeviceError> {
    let device: Device = {
        use crate::database::schema::device::dsl::*;
        use crate::database::schema::user;
        use diesel::prelude::*;
        let mut d: Vec<(Device, User)> = device
        .filter(token.eq(query_token))
        .inner_join(user::table)
        .get_results(conn)
        .map_err(|e| {
            match e {
                DieselError::NotFound => DeviceError::NotFound,
                _ => DeviceError::Unknown(e.to_string()),
            }
        })?;
        (&d.remove(0).0).clone()
    };

    Ok(device)
}