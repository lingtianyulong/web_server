use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::user_entity::schema::users;

#[derive(Debug, Queryable, Insertable)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub unregistered: i32,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub create_time: NaiveDateTime,
    pub unregistered: i32,
}

#[derive(AsChangeset, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub update_time: NaiveDateTime,
    pub unregistered: i32,
}

#[derive(AsChangeset, Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct DeleteUser<'a> {
    pub id: i64,
    pub user_name: &'a str,
    pub delete_time: NaiveDateTime,
    pub unregistered: i32,
}
