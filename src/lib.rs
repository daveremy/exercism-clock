use std::fmt;

#[derive(Debug, PartialEq, Clone)]
struct Hours(i32);

impl Hours {
    fn new(hours: i32) -> Hours {
        let hours = hours % 24;
        if hours < 0 {
            Hours(24 + hours)
        } else {
            Hours(hours)
        }
    }
}

impl fmt::Display for Hours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_fixed_2_char_string(self.0.to_string()))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Minutes(i32);

impl Minutes {
    fn new(minutes: i32) -> (Minutes, i32) {
        let hours_adjustment = minutes / 60;
        let minutes = minutes % 60;
        if minutes < 0 {
            (Minutes(60 + minutes), hours_adjustment - 1)
        } else {
            (Minutes(minutes), hours_adjustment)
        }
    }
}

impl fmt::Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_fixed_2_char_string(self.0.to_string()))
    }
}

fn to_fixed_2_char_string(s: String) -> String {
    match s.len() {
        2 => s,
        1 => format!("0{}", s),
        _ => panic!("String must be 1 or 2 chars"),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Clock {
    hours: Hours,
    minutes: Minutes,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (minutes, hours_adjustment) = Minutes::new(minutes);
        let hours = Hours::new(hours + hours_adjustment);
        Clock { hours, minutes }
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        Clock {
            hours: Hours(self.hours.0 + hours),
            minutes: self.minutes.clone(),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (new_minutes, hours_adjustment) = Minutes::new(minutes + self.minutes.0);
        let new_hours = Hours::new(self.hours.0 + hours_adjustment);
        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hours.to_string(), self.minutes.to_string())
    }
}
