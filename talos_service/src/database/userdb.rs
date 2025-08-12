use crate::database::dbutil;
use crate::entity::user;
use crate::logger;
use crate::utils::time_utils;

// 插入用户
pub async fn insert_user(user: &user::User) -> Result<(), sqlx::Error> {
    let db = dbutil::DButil::instance();
    let pool = db.pool();

    let _ =
        sqlx::query("INSERT INTO user (Id, user_name, password, create_time) VALUES (?, ?, ?, ?)")
            .bind(user.get_user_id())
            .bind(user.get_user_name())
            .bind(user.get_password())
            .bind(time_utils::get_current_time())
            .execute(pool)
            .await?;
    Ok(())
}

// 检查用户是否存在
pub async fn user_exist(user_name: &str) -> bool {
    let db = dbutil::DButil::instance();
    let pool = db.pool();
    let user_record = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM user WHERE user_name=?) AS result", user_name)
    .fetch_optional(pool)
    .await;

    match user_record {
        Ok(Some(record)) => record.result != 0,
        _ => false,
    }
}

// 根据用户名获取用户信息
pub async fn get_user_by_username(user_name: &str) -> Result<user::User, sqlx::Error> {
    let db = dbutil::DButil::instance();
    let pool = db.pool();

    let user_record = sqlx::query!(
        "SELECT Id, user_name, password, sex, age, phone, email, create_time, update_time 
        FROM user WHERE user_name = ?", user_name)
    .fetch_optional(pool)
    .await?;

    match user_record {
        Some(record) => {
            // 用户存在，创建 User 对象
            let user = user::User::create_user(
                record.Id,
                record.user_name,
                record.password,
                record.sex.unwrap_or_default(),
                record.age.unwrap_or(0) as u32,
                record.phone.unwrap_or_default(),
                record.email.unwrap_or_default(),
                record.create_time.unwrap_or(time_utils::get_current_time()),
                record.update_time,
            );

            Ok(user)
        }
        None => {
            // 用户不存在，记录日志并返回错误
            logger::error(&format!("User not found: {}", user_name));
            Err(sqlx::Error::RowNotFound)
        }
    }
}

// 根据用户名, 更新密码
// 返回受影响的行数
pub async fn update_password(user_name: &str, password: &str) -> Result<u64, sqlx::Error> {
    let db = dbutil::DButil::instance();
    let pool = db.pool();
    let result = sqlx::query("UPDATE user SET (password = ?, update_time = ?) WHERE user_name = ?")
        .bind(password)
        .bind(time_utils::get_current_time())
        .bind(user_name)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
