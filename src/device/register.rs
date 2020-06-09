use diesel::{MysqlConnection, RunQueryDsl};

use crate::database::model::{Device, NewDevice, User};
use crate::user::error::UserError;

pub fn register(
    conn: &MysqlConnection,
    user: User,
    device_name: String,
) -> Result<Device, UserError> {
    use crate::database::schema::device::dsl::*;
    use diesel::prelude::*;

    let new_device = NewDevice::new(user.id, device_name);
    let new_token = new_device.token.clone();

    diesel::insert_into(device)
    .values(new_device)
    .execute(conn)
    .map_err(|e| UserError::Unknown(e.to_string()))?;

    device
    .filter(token.eq(new_token))
    .get_result(conn)
    .map_err(|e| UserError::Unknown(e.to_string()))
}
