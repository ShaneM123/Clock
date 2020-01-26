pub struct Clock{hours: i32, minutes: i32}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours, minutes}
    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    pub fn to_string(&self) -> String {String::from(format!("{:02}:{:02}", self.hours, self.minutes))}
}


