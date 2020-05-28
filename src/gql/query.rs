use juniper::FieldResult;

use crate::gql::context::Context;
use crate::gql::enums::Episode;
use crate::gql::object::Human;

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
}