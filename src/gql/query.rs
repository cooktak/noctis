use juniper::FieldResult;

use crate::database::connection::MysqlPooledConnection;
use crate::device::{DeviceError, validate};
use crate::user::{error::UserError, local};

use super::context::Context;
use super::enums::Episode;
use super::object::{Device, Human, User};

pub struct QueryRoot;

#[juniper::object(
Context = Context,
)]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }

    fn user(context: &Context, username: String) -> FieldResult<User> {
        let conn: MysqlPooledConnection = match context.database_pool.get() {
            Ok(conn) => conn,
            Err(e) => Err(UserError::Unknown(e.to_string()))?,
        };
        let result = local::query(&conn, &username)?;
        Ok(User::from_database(&result))
    }

    fn validate_device(context: &Context, token: String) -> FieldResult<Device> {
        let conn: MysqlPooledConnection = match context.database_pool.get() {
            Ok(conn) => conn,
            Err(e) => Err(DeviceError::Unknown(e.to_string()))?,
        };
        let result = validate(&conn, token)?;
        Ok(Device::from_database(result))
    }
}