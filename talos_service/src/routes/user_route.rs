use crate::entity::user::{User, UserResponse};
use actix_web::{HttpResponse, Responder, post, web};

// 注册用户
#[post("/register")]
async fn register_user(user: web::Json<UserResponse>) -> impl Responder {
    let new_user = User::create_user_from_response(user.into_inner());
    HttpResponse::Ok().body(new_user.to_json())
}
