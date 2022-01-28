use core::time;

//
use anyhow::Result;
use chrono::{Utc, Date, DateTime, Duration, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset};
use chrono::TimeZone;

pub fn time_handler(prev: u32) -> NaiveDateTime {
    // こんな感じで時間は出せる。
    let now: DateTime<Utc> = Utc::now();
    let j_now = now + Duration::hours(9);
    let dt1 = j_now - Duration::days(prev.into());
    
    dt1.naive_utc()
}
fn main() {

    for i in 1..23 {
        let prev = time_handler(i);
        println!("Hello, world! {:?}", prev);
    }
    
    
}
