//
use chrono::{DateTime, Local, Duration, Weekday};
use chrono::Datelike;
use anyhow::Result;

pub fn get_weekday(days: u32) -> Vec<u32> {
    let date_management: Vec<u32> = (1..=days).into_iter().map(|x| x as u32).collect();

    let mut weekday: Vec<u32> = date_management.clone().into_iter().filter(|x| day_of_week(*x)).collect();

    while weekday.len() < 22 {
        let mut cursor: u32 = 1;
        let end_day = weekday.last().unwrap() + cursor;
        
        if day_of_week(end_day) {
            weekday.push(end_day);
        } else {
            cursor += 1;
            let back_1 = weekday.last().unwrap() + cursor;
            if day_of_week(back_1) {
                weekday.push(back_1);
            } else {
                cursor += 1;
                let back_2 = weekday.last().unwrap() + cursor;
                weekday.push(back_2);
            }
        }
    }

    weekday
}

fn day_of_week(x: u32) -> bool {
    let now: DateTime<Local> = chrono::offset::Local::now();

    let prev = now - Duration::days(x.into());
    let weekday = prev.date().weekday();
    match weekday {
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => true,
        Weekday::Sat | Weekday::Sun => false,
    }
}


fn main() -> Result<()> {
    
    let res = get_weekday(22);
    println!("{:?}", &res);
    Ok(())
}
