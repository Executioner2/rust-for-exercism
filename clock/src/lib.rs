use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::build(hours * 60 + minutes)
    }

    fn build(minutes: i32) -> Self {
        let mut minutes = minutes;
        while minutes < 0 {
            minutes += 1440;
        }
        Self {
            minutes: minutes % 1440,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::build(self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
