use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserDto<'a> {
    pub id: Option<i64>, // 添加 id 字段以支持更新操作，插入时为 None
    pub user_name: &'a str,
    pub password: &'a str,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub unregistered: i32,
}
