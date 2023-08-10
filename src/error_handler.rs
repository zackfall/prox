use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ProxError {
    line: u64,
    whre: String,
    message: String,
}

impl ProxError {
    pub fn new(line: u64, message: &str) -> ProxError {
        ProxError {
            line,
            whre: String::new(),
            message: message.to_string(),
        }
    }
}

impl Display for ProxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] Error{}: {}",
            self.line, self.whre, self.message
        )
    }
}

impl Error for ProxError {}
