pub struct Clock{hours: i32, minutes: i32}
use std::fmt;

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours, minutes}
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    pub fn to_string(&self) -> String {String::from(format!("{}",Clock{hours: self.hours, minutes: self.minutes}))}
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:02}:{:02}", (self.hours%24)+self.minutes/60, self.minutes%60)
            }
}


