use std::fmt::{Display, Formatter, Result};

const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hours = minutes.div_euclid(MINUTES_IN_HOUR);
        Clock {
            hours: (hours + extra_hours).rem_euclid(HOURS_IN_DAY),
            minutes: minutes - extra_hours * MINUTES_IN_HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
