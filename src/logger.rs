pub trait Logger {
    fn print(&mut self, value: String);
}

pub struct StringLogger(pub String);

impl StringLogger {
    pub fn new(st: String) -> Self {
        StringLogger(st)
    }
}

impl Logger for StringLogger {
    fn print(&mut self, value: String) {
        self.0.push_str(value.as_str());
    }
}
