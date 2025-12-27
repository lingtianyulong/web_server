use crate::db::UserDb;
use crate::entity::user_dto::UserDto;
use crate::route::request_data::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::route::request_data::{UpdateRequest, UpdateResponse, UnregisteredRequest, UnregisteredResponse};
use axum::{
    Extension, Json,
    extract::{Request, State},
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, HeaderName},
    },
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use encryption::argon2_encrypt::Argon2Encrypt;
use encryption::encrypt_trait::Encrypt;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use utils::time_util;

pub const SECRET_KEY: &[u8] = b"my-secret-key";
const TOKEN_TTL_SECS: i64 = 7 * 24 * 3600;
const REFRESHED_TOKEN_HEADER: &str = "x-refreshed-token";

/// 应用状态，用于在中间件中共享数据

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // 用户 ID
    pub exp: usize,       // 过期时间
    pub password: String, // 密码
}

/// 应用状态，用于在中间件中共享数据
#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret: Vec<u8>,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    // 先验证用户是否存在
    let user = match UserDb::get_user_by_username(&payload.user_name).await {
        Ok(v) => v,
        Err(e) => {
            let response = LoginResponse::new(false, e.to_string(), "".to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // 验证密码是否正确
    let argon2_encrypt = Argon2Encrypt::new();
    let is_valid = argon2_encrypt
        .verify(&payload.password, &user.password)
        .unwrap();
    if !is_valid {
        let response = LoginResponse::new(false, "Invalid password".to_string(), "".to_string());
        return (StatusCode::UNAUTHORIZED, Json(response));
    }

    // 直接使用 UTC 时间戳，避免时区混淆
    let exp = (Utc::now().timestamp() + TOKEN_TTL_SECS) as usize;

    let claims = Claims {
        sub: payload.user_name,
        password: payload.password,
        exp: exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .unwrap();

    let response = LoginResponse::new(true, "Login successful".to_string(), token);

    (StatusCode::OK, Json(response))
}

pub async fn get_user(Extension(claims): Extension<Claims>) -> (StatusCode, Json<LoginResponse>) {
    println!("claims: {:?}", claims);
    let response = LoginResponse::new(true, "Get user successful".to_string(), claims.sub);
    (StatusCode::OK, Json(response))
}

// 验证中间件
pub async fn jwt_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 动态放行规则（示例）
    let path = req.uri().path();
    if path.starts_with("/api/v1/public") {
        return Ok(next.run(req).await);
    }

    let auth = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 不校验 exp，便于后续自动刷新
    let mut validation = Validation::default();
    validation.validate_exp = false;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&state.jwt_secret),
        &validation,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = token_data.claims;
    let now_ts = Utc::now().timestamp() as usize;

    // 若已过期则自动刷新一次
    let mut active_claims = claims.clone();
    let mut refreshed_token: Option<String> = None;
    if now_ts >= claims.exp {
        let refreshed = Claims {
            sub: claims.sub.clone(),
            exp: (Utc::now().timestamp() + TOKEN_TTL_SECS) as usize,
            password: claims.password.clone(),
        };
        refreshed_token = Some(
            encode(
                &Header::default(),
                &refreshed,
                &EncodingKey::from_secret(&state.jwt_secret),
            )
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        );
        active_claims = refreshed;
    }

    // 注入 Claims，供 handler 使用
    req.extensions_mut().insert(active_claims);

    let mut response = next.run(req).await;

    if let Some(token) = refreshed_token {
        // 将刷新后的 token 通过响应头返回给客户端
        let header_name = HeaderName::from_static(REFRESHED_TOKEN_HEADER);
        let header_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        response.headers_mut().insert(header_name, header_value);
    }

    Ok(response)
}

pub async fn register_handler(
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<RegisterResponse>) {
    let create_time = match time_util::now() {
        Ok(tm) => tm,
        Err(e) => {
            let response = RegisterResponse::new(false, e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let user = UserDto {
        id: None, // 注册时 id 为 None
        user_name: payload.user_name.as_str(),
        password: payload.password.as_str(),
        create_time,
        update_time: None,
        delete_time: None,
        unregistered: 0,
    };

    match UserDb::insert(&user).await {
        Ok(res) => {
            if res > 0 {
                let response = RegisterResponse::new(true, "Register successful".to_string());
                (StatusCode::OK, Json(response))
            } else {
                let response = RegisterResponse::new(false, "Register failed".to_string());
                (StatusCode::OK, Json(response))
            }
        }
        Err(e) => {
            let response = RegisterResponse::new(false, e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn reset_password_handler(
    Json(payload): Json<UpdateRequest>,
) -> (StatusCode, Json<UpdateResponse>) {
    let user = match UserDb::get_user_by_username(&payload.user_name).await {
        Ok(v) => v,
        Err(e) => {
            let response = UpdateResponse::new(false, e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let update_time = match time_util::now() {
        Ok(tm) => tm,
        Err(e) => {
            let response = UpdateResponse::new(false, e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    let update_user = UserDto {
        id: Some(user.id), // 确保 id 存在
        user_name: user.user_name.as_str(),
        password: payload.password.as_str(),
        create_time: user.create_time,
        update_time: Some(update_time),
        delete_time: None,
        unregistered: 0,
    };

    match UserDb::update(&update_user).await {
        Ok(res) => {
            if res > 0 {
                let response = UpdateResponse::new(true, "Update successful".to_string());
                (StatusCode::OK, Json(response))
            } else {
                let response = UpdateResponse::new(false, "Update failed".to_string());
                (StatusCode::OK, Json(response))
            }
        }
        Err(e) => {
            let response = UpdateResponse::new(false, e.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}


pub async fn unregistered_handler(
    Json(payload): Json<UnregisteredRequest>,
) -> (StatusCode, Json<UnregisteredResponse>) {
    let user = match UserDb::get_user_by_username(&payload.user_name).await {
        Ok(v) => v,
        Err(e) => {
            let response = UnregisteredResponse::new(false, e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    match UserDb::delete(user.id).await {
        Ok(res) => {
            if res > 0 {
                let response = UnregisteredResponse::new(true, "Unregistered successful".to_string());
                (StatusCode::OK, Json(response))
            } else {
                let response = UnregisteredResponse::new(false, "Unregistered failed".to_string());
                (StatusCode::OK, Json(response))
            }
        }
        Err(e) => {
            let response = UnregisteredResponse::new(false, e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    }

}
