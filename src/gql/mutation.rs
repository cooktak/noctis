use juniper::FieldResult;
use log::error;
use thiserror::Error;

use crate::config::Database;
use crate::database::model::{NewUser as DatabaseNewUser, User as DatabaseUser};
use crate::gql::object::User;
use crate::user::local;

use super::context::Context;
use super::input::{NewHuman, NewUser};
use super::object::Human;

pub struct MutationRoot;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("Authentication error")]
    Authentication,
}

#[juniper::object(
Context = Context,
)]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }

    fn create_user(context: &Context, new_user: NewUser) -> Option<User> {
        let conn = context.database_pool
        .get()
        .map_err(|e| error!("Database Failed"))
        .ok()?;
        let result: DatabaseUser = local::create(&conn, DatabaseNewUser::from_graphql(new_user))
        .map_err(|e| UserError::NotFound).expect("Creation Error");
        Some(User::from_database(&result))
    }
}