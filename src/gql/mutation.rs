use juniper::FieldResult;

use crate::gql::context::Context;
use crate::gql::input::NewHuman;
use crate::gql::object::Human;

pub struct MutationRoot;

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
}