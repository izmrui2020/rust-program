use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Self {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Clock {
        Clock::new(0, &self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

struct Tmp {
    tmp: i64,
}

impl Tmp {
    fn new(f: i64) -> Self {
        Self { tmp: f }
    }
}
