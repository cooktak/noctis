use diesel::{MysqlConnection, result::Error as DieselError};

use crate::database::model::{Device, User};

use super::DeviceError;

pub fn devices(
    conn: &MysqlConnection,
    user: User,
) -> Result<Vec<Device>, DeviceError> {
    use crate::database::schema::device::dsl::*;
    use diesel::prelude::*;
    device
    .filter(user_id.eq(user.id))
    .load::<Device>(conn)
    .map_err(|e| {
        match e {
            DieselError::NotFound => DeviceError::NotFound,
            _ => DeviceError::Unknown(e.to_string()),
        }
    })
}