use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserDto<'a> {
    pub id: Option<i64>, // 添加 id 字段以支持更新操作，插入时为 None
    pub user_name: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub unregistered: i32,
}
