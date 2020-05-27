use juniper::FieldResult;

use crate::gql::schema::input::NewHuman;
use crate::gql::schema::object::Human;

pub struct MutationRoot;

#[juniper::object]
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