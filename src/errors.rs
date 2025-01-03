use std::fmt;

#[derive(Debug)]
pub struct YearFetchingError {
    pub message: String,
}

impl YearFetchingError {
    pub fn new(message: &str) -> Self {
        Self {
            message: format!("🚨 Year Fetching Operation Failed: {}", message),
        }
    }
}

impl fmt::Display for YearFetchingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for YearFetchingError {}
