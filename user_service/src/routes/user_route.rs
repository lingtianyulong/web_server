use crate::entity::user;
use crate::entity::user::UserResponse;
use crate::managers::dbmanager::DbManager;
use actix_web::{HttpResponse, Responder, post, web};
use logger;
use serde_json::{Value, json};

// 注册用户
#[post("/register")]
async fn register_user(user: web::Json<UserResponse>) -> impl Responder {
    let inner_user = user.into_inner();
    let new_user = user::User::create_user_from_response(inner_user);
    if new_user.get_user_name().is_empty() || new_user.get_password().is_empty() {
        logger::error("User name or password is empty");
        return HttpResponse::BadRequest().body("User name or password is empty");
    }

    let db_manager = match DbManager::instance().await {
        Ok(manager) => manager,
        Err(e) => {
            logger::error(&format!("Failed to get DbManager instance: {:#?}", e));
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": "Database connection failed",
                "data": false
            }));
        }
    };

    match db_manager.insert(&new_user).await {
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
async fn login_user(user: web::Json<serde_json::Value>) -> impl Responder {
    // 从输入的 json 中获取 user_name 和 password
    let Some(json_user_name) = user.get("user_name") else {
        logger::error("user_name field is missing");
        return HttpResponse::BadRequest().json(json!({
            "code": 0,
            "message": "user_name field is required",
        }));
    };

    let Some(json_password) = user.get("password") else {
        logger::error("password field is missing");
        return HttpResponse::BadRequest().json(json!({
            "code": 0,
            "message": "password field is required",
        }));
    };

    // 将 user_name 和 password 转换为字符串
    let user_name = match json_user_name.as_str() {
        Some(username) => username,
        None => {
            logger::error("user_name is not a string");
            return HttpResponse::BadRequest().json(json!({
                "code": 0,
                "message": "user_name must be a string",
            }));
        }
    };

    let password = match json_password.as_str() {
        Some(password) => password,
        None => {
            logger::error("password is not a string");
            return HttpResponse::BadRequest().json(json!({
                "code": 0,
                "message": "password must be a string",
            }));
        }
    };

    let db_manager = match DbManager::instance().await {
        Ok(manager) => manager,
        Err(e) => {
            logger::error(&format!("Failed to get DbManager instance: {:#?}", e));
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": "Database connection failed",
                "data": false
            }));
        }
    };

    let user = match db_manager.find::<user::User, &str>("user_name", user_name).await {
        Ok(user) => user,
        Err(e) => {
            logger::error(&format!("Failed to find user: {:#?}", e));
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": "Database operation failed",
                "data": false
            }));
        }
    };

    if user.get_password() != password {
        return HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "password is incorrect",
            "data": false
        }));
    }

    HttpResponse::Ok().body(user.to_json())
}

// 判断用户是否存在
#[post("/user_exist")]
async fn user_exist(req_body: web::Json<serde_json::Value>) -> impl Responder {
    let Some(json_user_name) = req_body.get("user_name") else {
        logger::error("user_name field is missing");
        return HttpResponse::BadRequest().json(json!({
            "code": 0,
            "message": "user_name field is required",
        }));
    };

    let user_name = match json_user_name.as_str() {
        Some(username) => username,
        None => {
            logger::error("user_name is not a string");
            return HttpResponse::BadRequest().json(json!({
                "code": 0,
                "message": "user_name must be a string",
            }));
        }
    };

    let db_manager = match DbManager::instance().await {
        Ok(manager) => manager,
        Err(e) => {
            logger::error(&format!("Failed to get DbManager instance: {:#?}", e));
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": "Database connection failed",
                "data": false
            }));
        }
    };

    let Ok(exists) = db_manager
        .exists::<user::User>("user_name = ?", vec![&Value::String(user_name.to_string())])
        .await
    else {
        logger::error("Failed to check user existence");
        return HttpResponse::InternalServerError().json(json!({
            "code": -1,
            "message": "Database operation failed",
            "data": false
        }));
    };

    if exists {
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
    let Some(json_user_name) = req_body.get("user_name") else {
        logger::error("user_name field is missing");
        return HttpResponse::BadRequest().json(json!({
            "code": 0,
            "message": "user_name field is required",
        }));
    };

    // 获取密码
    let Some(json_password) = req_body.get("password") else {
        logger::error("password field is missing");
        return HttpResponse::BadRequest().json(json!({
            "code": 0,
            "message": "password field is required",
        }));
    };

    let user_name = match json_user_name.as_str() {
        Some(username) => username,
        None => {
            logger::error("user_name is not a string");
            return HttpResponse::BadRequest().json(json!({
                "code": 0,
                "message": "user_name must be a string",
            }));
        }
    };

    let password = match json_password.as_str() {
        Some(password) => password,
        None => {
            logger::error("password is not a string");
            return HttpResponse::BadRequest().json(json!({
                "code": 0,
                "message": "password must be a string",
            }));
        }
    };

    let db_manager = match DbManager::instance().await {
        Ok(manager) => manager,
        Err(e) => {
            logger::error(&format!("Failed to get DbManager instance: {:#?}", e));
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": "Database connection failed",
                "data": false
            }));
        }
    };

    let user: Option<user::User> = match db_manager.find::<user::User, &str>("user_name", user_name).await {
        Ok(user) => Some(user),
        Err(e) => {
            let err_msg = format!("Failed to find user: {:#?}", e);
            logger::error(&err_msg.as_str());
            return HttpResponse::InternalServerError().json(json!({
                "code": -1,
                "message": err_msg.as_str(),
                "data": false
            }));
        }
    };

    if user.is_none() {
        logger::error("user not found");
        return HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "user not found",
            "data": false
        }));
    }

    let mut user_obj = user.unwrap();
    user_obj.set_password(password);
    
    let result = db_manager.update(&user_obj, "user_name").await;
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
