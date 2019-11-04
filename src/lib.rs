use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        Clock {
            minutes: (hours * 60) + minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock {
            minutes: self.minutes + minutes,
        }
    }

    fn get_hours_minutes(&self) -> (u32, u32) {
        (
            self.minutes.div_euclid(60).rem_euclid(24) as u32,
            self.minutes.rem_euclid(60) as u32,
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.get_hours_minutes() == other.get_hours_minutes()
    }
}
impl Eq for Clock {}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes) = self.get_hours_minutes();
        write!(f, "{:0width$}:{:0width$}", hours, minutes, width = 2)
    }
}
