use crate::route::request_data::{LoginRequest, LoginResponse};
use axum::{
    Extension, Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

pub const SECRET_KEY: &[u8] = b"my-secret-key";

/// 应用状态，用于在中间件中共享数据

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// 应用状态，用于在中间件中共享数据
#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret: Vec<u8>,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    // 直接使用 UTC 时间戳，避免时区混淆
    let exp = (Utc::now().timestamp() + 3600) as usize;

    let claims = Claims {
        sub: payload.user_name,
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

pub async fn get_user(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    let response = LoginResponse::new(true, "Get user successful".to_string(), claims.sub);
    Json(response)
}

// 验证中间件
pub async fn jwt_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    use axum::http::header::AUTHORIZATION;

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

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&state.jwt_secret),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?
    .claims;

    // 注入 Claims，供 handler 使用
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
