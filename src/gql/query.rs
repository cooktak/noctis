use juniper::FieldResult;
use log::error;

use crate::gql::context::Context;
use crate::gql::enums::Episode;
use crate::gql::mutation::UserError;
use crate::gql::object::{Human, User};
use crate::user::local;

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

    fn user(context: &Context, username: String, user_tag: i32) -> Option<User> {
        let conn = context.database_pool
        .get()
        .map_err(|e| error!("Database Failed"))
        .ok()?;
        let result = local::query(&conn, &username, user_tag)
        .map_err(|e| UserError::NotFound)
        .expect("User Not Found");
        Some(User::from_database(&result))
    }
}