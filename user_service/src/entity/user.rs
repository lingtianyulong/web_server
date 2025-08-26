use chrono::NaiveDateTime;
use model_derive::Model as DeriveModel;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::database::Model;
use utils::times;
use uuid::Uuid;

// 向数据库存储的用户数据
#[derive(Debug, DeriveModel, Serialize, Deserialize, FromRow)]
#[table_name = "user"]
pub struct User {
    id: String,                         // 用户id, 唯一标识
    user_name: String,                  // 用户名
    password: String,                   // 密码
    sex: String,                        // 性别
    age: u32,                           // 年龄
    phone: String,                      // 手机号
    email: String,                      // 邮箱
    create_time: NaiveDateTime,         // 创建时间
    update_time: Option<NaiveDateTime>, // 更新时间
}

// 前端输入或返回的用户数据
#[derive(Debug, Deserialize)]
pub struct UserResponse {
    user_name: String, // 用户名
    password: String,  // 密码
    sex: String,       // 性别
    age: u32,          // 年龄
    phone: String,     // 手机号
    email: String,     // 邮箱
}

#[allow(dead_code)]
impl User {
    pub fn new(
        user_name: String,
        password: String,
        sex: String,
        age: u32,
        phone: String,
        email: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_name,
            password,
            sex,
            age,
            phone,
            email,
            create_time: times::get_current_time(),
            update_time: None,
        }
    }

    pub fn create_user(
        id: String,
        user_name: String,
        password: String,
        sex: String,
        age: u32,
        phone: String,
        email: String,
        create_time: NaiveDateTime,
        update_time: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            user_name,
            password,
            sex,
            age,
            phone,
            email,
            create_time,
            update_time,
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.id
    }

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_sex(&self) -> &str {
        &self.sex
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(self) {
            Ok(json) => json,
            Err(e) => {
                logger::error(&format!("Failed to convert user to json: {}", e));
                "".to_string()
            }
        }
    }

    pub fn create_user_from_response(user_response: impl Into<UserResponse>) -> Self {
        let user_response = user_response.into();
        Self {
            id: Uuid::new_v4().to_string(),
            user_name: user_response.user_name,
            password: user_response.password,
            sex: user_response.sex,
            age: user_response.age,
            phone: user_response.phone,
            email: user_response.email,
            create_time: times::get_current_time(),
            update_time: None,
        }
    }

    // 从数据库记录创建用户（仅基本字段）
    pub fn from_db_record(id: String, user_name: String, password: String) -> Self {
        Self {
            id,
            user_name,
            password,
            sex: "".to_string(),
            age: 0,
            phone: "".to_string(),
            email: "".to_string(),
            create_time: times::get_current_time(),
            update_time: None,
        }
    }
}
