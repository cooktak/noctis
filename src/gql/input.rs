use juniper::GraphQLInputObject;

use crate::gql::enums::Episode;
use chrono::NaiveDateTime;
use crate::database::enums::Gender;

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New User")]
pub struct NewUser {
    pub birthday: NaiveDateTime,
    pub gender: Gender,
    pub nickname: String,
    pub password: String,
    pub username: String,
}