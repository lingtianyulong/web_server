use crate::logger;
use crate::utils::time_utils;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    SuperAdmin, // 超级管理员
    Admin,      // 管理员
    User,       // 用户
    Guest,      // 游客
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatus {
    Active,   // 活跃
    Inactive, // 不活跃
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysUser {
    id: String,
    user_name: String,                          // 用户名
    password: String,                           // 密码
    phone: String,                              // 手机号
    role: UserRole,                             // 角色
    status: UserStatus,                         // 状态
    create_time: NaiveDateTime,                 // 创建时间
    update_time: Option<NaiveDateTime>,         // 更新时间
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysUserResponse {
    user_name: String,  
    password: String,
    phone: String,
    role: UserRole,
    status: UserStatus,
}

impl Default for SysUser {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_name: "".to_string(),
            password: "".to_string(),
            phone: "".to_string(),
            role: UserRole::User,
            status: UserStatus::Active,
            create_time: time_utils::get_current_time(),
            update_time: None,
        }
    }
}

impl SysUser {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_name,
            password,
            phone: "".to_string(),
            role: UserRole::User,
            status: UserStatus::Active,
            create_time: time_utils::get_current_time(),
            update_time: None,
        }
    }

    pub fn create_user_from_response(user_response: impl Into<SysUserResponse>) -> Self {
        let user_response = user_response.into();
        Self {
            id: Uuid::new_v4().to_string(),
            user_name: user_response.user_name,
            password: user_response.password,
            phone: user_response.phone,
            role: user_response.role,
            status: user_response.status,
            create_time: time_utils::get_current_time(),
            update_time: None,
        }
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

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_role(&self) -> &UserRole {
        &self.role
    }
    pub fn set_role(&mut self, role: UserRole) {
        self.role = role;
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }
}
