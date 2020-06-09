use chrono::NaiveDateTime;
use juniper::GraphQLInputObject;

use super::enums::Gender;

#[derive(GraphQLInputObject)]
#[graphql(description = "New User")]
pub struct NewUser {
    pub birthday: NaiveDateTime,
    pub gender: Gender,
    pub nickname: String,
    pub password: String,
    pub username: String,
    pub user_tag: Option<i32>,
}