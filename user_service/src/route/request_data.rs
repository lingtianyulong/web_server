use serde::{Deserialize, Serialize};

/// 用户登录请求结构体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[allow(dead_code)]
impl LoginRequest {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            user_name,
            password,
        }
    }
}

/// 用户登录响应结构体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoginResponse {
    pub success: bool,   // 是否登录成功
    pub message: String, // 登录失败时的错误信息
    pub token: String,   // 登录成功后的token
}

#[allow(dead_code)]
impl LoginResponse {
    pub fn new(success: bool, message: String, token: String) -> Self {
        Self {
            success,
            message,
            token,
        }
    }
}

/// 用户注册请求结构体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RegisterRequest {
    pub user_name: String,
    pub password: String,
}

#[allow(dead_code)]
impl RegisterRequest {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            user_name,
            password,
        }
    }
}

/// 用户注册响应结构体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

#[allow(dead_code)]
impl RegisterResponse {
    pub fn new(success: bool, message: String) -> Self {
        Self {
            success,
            message,
        }
    }
}