use chrono::NaiveDateTime;
use diesel_derive_enum::*;

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

#[derive(Debug, DbEnum)]
pub enum UserGender {
    Private,
    Male,
    Female,
    Etc,
}

#[derive(Queryable)]
pub struct User {
    id: i32,
    birthday: NaiveDateTime,
    create_time: NaiveDateTime,
    gender: UserGender,
    nickname: String,
    password: String,
    username: String,
    user_tag: i32,
    photo_link: Option<String>,
}
