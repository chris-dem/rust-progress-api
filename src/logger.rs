use std::fmt::Arguments;

pub trait Logger {
    fn print(&mut self, value: &Arguments<'_>);
}

pub struct StringLogger(pub String);

impl StringLogger {
    pub fn new(st: String) -> Self {
        StringLogger(st)
    }
}

impl Logger for StringLogger {
    fn print(&mut self, value: &Arguments<'_>) {
        self.0.push_str(format!("{}", value).as_str());
    }
}
