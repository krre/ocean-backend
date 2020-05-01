use std::collections::HashMap;
use std::error;
use std::fmt;

pub type ErrorCode = i32;

// Common (1..99)
pub const PARSE_ERROR: ErrorCode = 1;
pub const CONTROLLER_NOT_FOUND: ErrorCode = 2;
pub const METHOD_NOT_FOUND: ErrorCode = 3;
pub const PARAMETER_NOT_FOUND: ErrorCode = 4;

// User (100..199)
pub const INVALID_PAIR_ID_PASSWORD_: ErrorCode = 100;

lazy_static! {
    static ref ERROR_MESSAGES: HashMap<ErrorCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(PARSE_ERROR, "Parse error");
        m.insert(CONTROLLER_NOT_FOUND, "Controller not found");
        m.insert(METHOD_NOT_FOUND, "Method not found");
        m.insert(PARAMETER_NOT_FOUND, "Parameter not found");
        m.insert(INVALID_PAIR_ID_PASSWORD_, "Invalid pair `id / password`");
        m
    };
}

#[derive(Debug, Clone)]
pub struct Error {
    code: ErrorCode,
    message: String,
    data: Option<String>,
}

impl Error {
    pub fn new(code: ErrorCode, data: Option<String>) -> Self {
        Error {
            code,
            message: ERROR_MESSAGES.get(&code).unwrap().to_string(),
            data,
        }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "API error: code: {}, message: {}",
            self.code, self.message
        )
    }
}

impl error::Error for Error {}