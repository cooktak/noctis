use chrono::{DateTime, Utc};
use juniper::GraphQLObject;

use crate::database::model::{Device as DatabaseDevice, User as DatabaseUser};

use super::enums::{Episode, Gender};

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
    birthday: DateTime<Utc>,
    create_time: DateTime<Utc>,
    gender: Gender,
    nickname: String,
    username: String,
    user_tag: i32,
    photo_link: Option<String>,
}

impl User {
    pub fn from_database(user: &DatabaseUser) -> Self {
        Self {
            birthday: DateTime::from_utc(user.birthday, Utc),
            create_time: DateTime::from_utc(user.create_time, Utc),
            gender: match user.gender.as_str() {
                "private" => Gender::Private,
                "male" => Gender::Male,
                "female" => Gender::Female,
                _ => Gender::Etc,
            },
            nickname: user.nickname.clone(),
            username: user.username.clone(),
            user_tag: user.user_tag.clone(),
            photo_link: user.photo_link.clone(),
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Device Registration")]
pub struct Device {
    pub id: i32,
    pub token: String,
    pub name: String,
    pub user_id: i32,
}

impl Device {
    pub fn from_database(device: &DatabaseDevice) -> Self {
        Self {
            id: device.id,
            token: device.token.clone(),
            name: device.name.clone(),
            user_id: device.user_id,
        }
    }
}
