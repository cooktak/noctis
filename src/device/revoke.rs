use diesel::{MysqlConnection, RunQueryDsl};

use crate::database::model::Device;

use super::DeviceError;

pub fn revoke(
    conn: &MysqlConnection,
    target_token: String,
) -> Result<Device, DeviceError> {
    use crate::database::schema::device::dsl::*;
    use diesel::prelude::*;

    let target_device = super::validate(conn, target_token.clone())?;

    diesel::delete(device.filter(token.eq(target_token)))
    .execute(conn)
    .map_err(|e| {
        DeviceError::Unknown(e.to_string())
    })?;

    Ok(target_device)
}
