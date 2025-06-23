pub use chrono::Weekday as wd;
use chrono::{Datelike, Duration, NaiveDate};

pub fn middle_day(year: i32) -> Option<wd> {
    let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap();
    let total_days = (end - start).num_days();

    if total_days % 2 != 0 {
        let middle_day = start + Duration::days(total_days / 2);
        Some(middle_day.weekday())
    } else {
        None
    }
}