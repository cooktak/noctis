use chrono::NaiveDateTime;

use crate::gql::{enums::Gender, input::NewUser as GraphQLNewUser};

use super::schema::user;

#[derive(Queryable)]
pub struct Cookery {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct FoodType {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub create_time: NaiveDateTime,
    pub title: String,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct PostContent {
    pub id: i32,
    pub content: String,
    pub photo_link: String,
    pub post_id: i32,
}

#[derive(Queryable)]
pub struct PostCookery {
    pub post_id: i32,
    pub cookery_id: i32,
}

#[derive(Queryable)]
pub struct PostFoodType {
    pub post_id: i32,
    pub food_type_id: i32,
}

#[derive(Queryable)]
pub struct PostIngredient {
    pub post_id: i32,
    pub ingredient_id: i32,
    pub amount: i32,
    pub unit: i32,
}

#[derive(Queryable)]
pub struct PostProduct {
    pub post_id: i32,
    pub product_id: i32,
}

#[derive(Queryable)]
pub struct Product {
    pub id: String,
    pub info: String,
    pub kan_product_category_code: String,
    pub name: String,
    pub photo_link: Option<String>,
    pub unit: String,
    pub wight: String,
    pub seller_id: i32,
    pub vendor_id: i32,
}

#[derive(Queryable)]
pub struct Seller {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Token {
    pub id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: i32,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub birthday: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub gender: String,
    pub nickname: String,
    pub password: String,
    pub username: String,
    pub user_tag: i32,
    pub photo_link: Option<String>,
}

impl User {
    pub fn hashed_password(password: &str, username: &str) -> String {
        argon2rs::argon2i_simple(
            password,
            format!("{}@cooktak", username).as_str(),
        )
        .to_vec()
        .into_iter()
        .map(|v| v as char)
        .collect()
    }
}

#[derive(Insertable, Debug)]
#[table_name = "user"]
pub struct NewUser {
    pub birthday: NaiveDateTime,
    pub gender: String,
    pub nickname: String,
    pub password: String,
    pub username: String,
    pub user_tag: i32,
}


impl NewUser {
    pub fn from_graphql(user: GraphQLNewUser) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            birthday: user.birthday,
            gender: match user.gender {
                Gender::Private => String::from("private"),
                Gender::Male => String::from("male"),
                Gender::Female => String::from("female"),
                _ => String::from("etc"),
            },
            nickname: user.nickname,
            password: user.password,
            username: user.username,
            user_tag: match user.user_tag {
                Some(tag) => tag,
                None => rng.gen_range(1, 9999),
            },
        }
    }

    pub fn to_hashed(&self) -> Self {
        NewUser {
            birthday: self.birthday.clone(),
            gender: self.gender.clone(),
            nickname: self.nickname.clone(),
            password: User::hashed_password(
                &self.password,
                &self.username,
            ),
            username: self.username.clone(),
            user_tag: self.user_tag.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn password_hashing() {
        let user: NewUser = NewUser {
            birthday: NaiveDate::from_ymd(2002, 1, 22).and_hms(0, 0, 0),
            gender: "etc".to_string(),
            nickname: "SLoWMoTIoN".to_string(),
            password: "P@ssw0rd".to_string(),
            username: "username@domain.com".to_string(),
            user_tag: 122,
        };
        assert_eq!(
            user.to_hashed().password,
            User::hashed_password(&user.password, &user.username)
        );
    }
}
