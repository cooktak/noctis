use juniper::FieldResult;
use log::error;
use thiserror::Error;

use crate::database::model::{
    Device as DatabaseDevice,
    NewUser as DatabaseNewUser,
    User as DatabaseUser,
};
use crate::device;
use crate::user::local;

use super::context::Context;
use super::input::{NewHuman, NewUser};
use super::object::{Device, Human, User};

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

    fn create_user(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let conn = context.database_pool.get()?;
        let result: DatabaseUser = local::create(&conn, DatabaseNewUser::from_graphql(new_user))?;
        Ok(User::from_database(&result))
    }

    fn register_device(context: &Context, username: String, device_name: String) -> FieldResult<Device> {
        let conn = context.database_pool.get()?;

        let user = local::query(&conn, &username)?;

        let result: DatabaseDevice = device::register(&conn, user, device_name)?;
        Ok(Device::from_database(&result))
    }

    fn revoke_device(context: &Context, token: String) -> FieldResult<Device> {
        let conn = context.database_pool.get()?;

        let result: DatabaseDevice = device::revoke(&conn, token)?;
        Ok(Device::from_database(&result))
    }
}