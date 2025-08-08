use chrono::{ DateTime, FixedOffset, Utc, NaiveDateTime};

pub fn get_current_time() -> NaiveDateTime {
    let now = Utc::now();
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let datetime: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(now.naive_local(), offset);
    let format = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let datetime = NaiveDateTime::parse_from_str(&format, "%Y-%m-%d %H:%M:%S").unwrap();
    datetime
}