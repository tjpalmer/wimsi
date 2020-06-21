use std::fmt;

#[derive(Clone, Debug)]
pub struct Error {
    message: &'static str,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

pub fn err(message: &'static str) -> Box<dyn std::error::Error> {
    let error = Error { message };
    Box::new(error)
}

pub type Try<Value> = Result<Value, Box<dyn std::error::Error>>;
