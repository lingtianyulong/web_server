use chrono::{Timelike, Utc, Duration};
use std::error::Error;
use chrono::NaiveDateTime;

/**
 * 获取当前北京时间
 * @return 当前北京时间
 */
pub fn now() -> Result<NaiveDateTime, Box<dyn Error>> {
    // 获取当前UTC时间，并加上8小时时差，得到北京时间
    let beijing_time = Utc::now().naive_local() + Duration::hours(8);
    // 将纳秒部分设置为0，得到北京时间
    let tm = match beijing_time.with_nanosecond(0) {
        Some(time) => time,
        None => {
            let msg = format!("Failed to get current time");
            return Err(msg.into());
        }
    };
    Ok(tm)
}

#[cfg(test)]
mod tests {
    use super::now;
    use chrono::{Duration, Timelike, Utc};
    #[test]
    fn test_now() {
        let tm1 = (Utc::now().naive_local() + Duration::hours(8)).with_nanosecond(0).unwrap();
        let tm2 = now().unwrap();
        println!("tm1: {}", tm1);
        println!("tm2: {}", tm2);
        assert!(tm1 == tm2);
    }
}
