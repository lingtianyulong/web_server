use chrono::{DateTime, Utc, TimeZone, FixedOffset};

pub fn get_current_time() -> DateTime<Utc> {
    let now = Utc::now();
    let offset = FixedOffset::east(8 * 3600);
    let time = now.with_timezone(&offset);
    time
}