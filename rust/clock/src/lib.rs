use std::fmt;
const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Clock {
            //to make it positive
            // minutes: (((hours * HOUR + minutes) % DAY + DAY) % DAY),
            minutes: (hours * HOUR + minutes).rem_euclid(DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
