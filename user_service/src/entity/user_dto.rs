use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserDto<'a> {
    pub id: i64,
    pub user_name: &'a str,
    pub password: &'a str,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub unregistered: i32,
}