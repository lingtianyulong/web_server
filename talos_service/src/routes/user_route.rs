use crate::database::userdb;
use crate::entity::user;
use crate::entity::user::{User, UserResponse};
use crate::logger;
use actix_web::{HttpResponse, Responder, post, web};
use serde_json::json;

// 注册用户
#[post("/register")]
async fn register_user(user: web::Json<UserResponse>) -> impl Responder {
    let inner_user = user.into_inner();
    let new_user = user::User::create_user_from_response(inner_user);
    if new_user.get_user_name().is_empty() || new_user.get_password().is_empty() {
        logger::error("User name or password is empty");
        return HttpResponse::BadRequest().body("User name or password is empty");
    }

    match userdb::insert_user(&new_user).await {
        Ok(_) => {
            logger::info("insert user success!");
            logger::info(&format!("User created: {:#?}", new_user));
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "User register success",
                "data": true
            }))
        }
        Err(e) => {
            logger::error(&format!("Failed to create user: {:#?}", e));
            HttpResponse::Ok().json(json!({
                "code": 0,
                "message": "User register failed",
                "data": false
            }))
        }
    }
}

// 用户登录
#[post("/login")]
async fn login_user(user: web::Json<UserResponse>) -> impl Responder {
    let new_user = User::create_user_from_response(user.into_inner());
    HttpResponse::Ok().body(new_user.to_json())
}

// 判断用户是否存在
#[post("/user_exist")]
async fn user_exist(req_body: web::Json<serde_json::Value>) -> impl Responder {
    let user_name = match req_body.get("user_name") {
        Some(name) => match name.as_str() {
            Some(name_str) => name_str,
            None => {
                logger::error("user_name is not a string");
                return HttpResponse::BadRequest().json(json!({
                    "code": -1,
                    "message": "user_name must be a string"
                }));
            }
        },
        None => {
            logger::error("user_name field is missing");
            return HttpResponse::BadRequest().json(json!({
                "code": -1,
                "message": "user_name field is required"
            }));
        }
    };
    
    let exist = userdb::user_exist(user_name).await;
    if exist {
        HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "User exists",
            "data": true
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "User not exists",
            "data": false
        }))
    }
}

// 重置密码
#[post("/reset_password")]
async fn reset_password(req_body: web::Json<serde_json::Value>) -> impl Responder {
    // 获取用户名
    let user_name = match req_body.get("user_name") {
        Some(name) => match name.as_str() {
            Some(name_str) => name_str,
            None => {
                logger::error("user_name is not a string");
                return HttpResponse::BadRequest().json(json!({
                    "code": -1,
                    "message": "user_name must be a string"
                }));
            }
        },
        None => {
            logger::error("user_name field is missing");
            return HttpResponse::BadRequest().json(json!({
                "code": -1,
                "message": "user_name field is required"
            }));
        }
    };

    // 获取密码
    let password: &str = match req_body.get("password") {
        Some(password) => match password.as_str() {
            Some(password_str) => password_str,
            None => {
                logger::error("password is not a string");
                return HttpResponse::BadRequest().json(json!({
                    "code": -1,
                    "message": "password must be a string",
                    "data": false
                }));
            }
        },
        None => {
            logger::error("password field is missing");
            return HttpResponse::BadRequest().json(json!({
                "code": -1,
                "message": "password field is required",
                "data": false
            }));
        }
    };

    let exist = userdb::user_exist(user_name).await;
    if !exist {
        logger::error("user not found");
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "message": "user not found",
            "data": false
        }));
    }

    let result = userdb::update_password(user_name, password).await;
    if result.is_err() {
        logger::error("Failed to update password");
        return HttpResponse::Ok().json(json!({
            "code": -1,
            "message": "password reset failed",
            "data": false
        }));
    }

    HttpResponse::Ok().json(json!({
        "code": 0,
        "message": "password reset success",
        "data": true
    }))

}
