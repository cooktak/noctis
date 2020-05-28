use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::database::model::{NewUser, User};

pub fn create(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> Option<User> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;
    diesel::insert_into(user).values(&new_user).execute(conn);
    let mut q: std::vec::Vec<User> = user
    .filter(username.eq(new_user.username.to_owned()))
    .load::<User>();
    q.pop()
}