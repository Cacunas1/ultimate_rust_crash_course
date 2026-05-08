use std::fmt;

const MAX_HOURS: i32 = 24;
const MAX_MINUTES: i32 = 60;
const HOUR_TO_MINUTES: i32 = MAX_MINUTES;
const MAX_TIME: i32 = MAX_HOURS * HOUR_TO_MINUTES;


#[derive(Debug)]
pub struct Clock {
    internal_time: i32,
}

impl Clock {
    fn calculate_internal_time(hours: i32, minutes: i32) -> i32 {
        let mut internal_time: i32 = hours * HOUR_TO_MINUTES + minutes;
        if (internal_time >= MAX_TIME) || (internal_time < 0) {
            internal_time = internal_time.rem_euclid(MAX_TIME);
        }
        internal_time
    }

    fn from_internal_time(internal_time: i32) -> Self {
        let mut aux_time: i32 = internal_time;
        if (aux_time > MAX_TIME) || (aux_time < 0) {
            aux_time = aux_time.rem_euclid(MAX_TIME);
        }
        Self {
            internal_time: aux_time,
        }
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = Self::calculate_internal_time(hours, minutes);
        Self::from_internal_time(time)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = Self::calculate_internal_time(0i32, minutes);
        Self::from_internal_time(self.internal_time + time)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours: i32 = self.internal_time.div_euclid(HOUR_TO_MINUTES);
        let minutes: i32 = self.internal_time.rem_euclid(HOUR_TO_MINUTES);
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.internal_time == other.internal_time
    }
}
