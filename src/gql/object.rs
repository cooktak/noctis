use chrono::NaiveDateTime;
use juniper::GraphQLObject;

use crate::database::{enums::Gender, model::User as DatabaseUser};
use crate::gql::enums::Episode;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "User")]
pub struct User {
    birthday: NaiveDateTime,
    create_time: NaiveDateTime,
    gender: Gender,
    nickname: String,
    username: String,
    user_tag: i32,
    photo_link: Option<String>,
}

impl User {
    pub fn from_database(user: DatabaseUser) -> Self {
        Self {
            birthday: user.birthday,
            create_time: user.create_time,
            gender: user.gender,
            nickname: user.nickname,
            username: user.username,
            user_tag: user.user_tag,
            photo_link: user.photo_link,
        }
    }
}
