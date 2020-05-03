use std::collections::HashMap;
use std::error;
use std::fmt;

pub type ErrorCode = i32;

// Common (1..99)
pub const PARSE_ERROR: ErrorCode = 1;
pub const CONTROLLER_NOT_FOUND: ErrorCode = 2;
pub const METHOD_NOT_FOUND: ErrorCode = 3;
pub const PARAMETER_NOT_FOUND: ErrorCode = 4;
pub const INTERNAL_SERVER_ERROR: ErrorCode = 5;

// User (100..199)
pub const WRONG_USER_PASSWORD: ErrorCode = 100;

lazy_static! {
    static ref ERROR_MESSAGES: HashMap<ErrorCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(PARSE_ERROR, "Parse error");
        m.insert(CONTROLLER_NOT_FOUND, "Controller not found");
        m.insert(METHOD_NOT_FOUND, "Method not found");
        m.insert(PARAMETER_NOT_FOUND, "Parameter not found");
        m.insert(INTERNAL_SERVER_ERROR, "Internal server error");

        m.insert(WRONG_USER_PASSWORD, "Wrong user password`");
        m
    };
}

#[derive(Debug)]
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

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "API error: code: {}, message: {}",
            self.code, self.message
        )
    }
}

pub fn make_error(code: ErrorCode) -> Box<dyn error::Error> {
    Box::new(Error::new(code, None))
}

pub fn make_error_data(code: ErrorCode, data: String) -> Box<dyn error::Error> {
    Box::new(Error::new(code, Some(data)))
}
