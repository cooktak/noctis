use juniper::{FieldError, FieldResult};
use log::error;

use crate::database::connection::MysqlPooledConnection;
use crate::user::{error::UserError, local};

use super::context::Context;
use super::enums::Episode;
use super::object::{Human, User};

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

    fn user(context: &Context, username: String, user_tag: i32) -> FieldResult<User> {
        let conn: MysqlPooledConnection = match context.database_pool.get() {
            Ok(conn) => conn,
            Err(e) => Err(UserError::Unknown(e.to_string()))?,
        };
        let result = local::query(&conn, &username, user_tag)?;
        Ok(User::from_database(&result))
    }
}