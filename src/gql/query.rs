use juniper::FieldResult;

use crate::database::connection::MysqlPooledConnection;
use crate::device::{self, DeviceError};
use crate::user::{error::UserError, local};

use super::context::Context;
use super::object::{Device, User};

pub struct QueryRoot;

#[juniper::object(
Context = Context,
)]
impl QueryRoot {
    fn user(context: &Context, username: String) -> FieldResult<User> {
        let conn: MysqlPooledConnection = match context.database_pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(UserError::Unknown(e.to_string()).into()),
        };
        let result = local::query(&conn, &username)?;
        Ok(User::from_database(&result))
    }

    fn validate_device(context: &Context, token: String) -> FieldResult<Device> {
        let conn: MysqlPooledConnection = match context.database_pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(DeviceError::Unknown(e.to_string()).into()),
        };
        let result = device::validate(&conn, token)?;
        Ok(Device::from_database(&result))
    }

    fn obtain_devices(context: &Context, username: String) -> FieldResult<Vec<Device>> {
        let conn: MysqlPooledConnection = context.database_pool.get()?;

        let user = local::query(&conn, &username)?;

        let result = device::devices(&conn, user)?;

        Ok(result.iter().map(|e| Device::from_database(e)).collect())
    }
}