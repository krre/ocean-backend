use std::collections::HashMap;
use std::error;
use std::fmt;
use std::sync::LazyLock;

pub type ErrorCode = i32;

// Common (1..99)
pub const PARSE_ERROR: ErrorCode = 1;
pub const CONTROLLER_NOT_FOUND: ErrorCode = 2;
pub const METHOD_NOT_FOUND: ErrorCode = 3;
pub const PARAMETER_NOT_FOUND: ErrorCode = 4;
pub const INTERNAL_SERVER_ERROR: ErrorCode = 5;
pub const INVALID_PARAMETER: ErrorCode = 6;
pub const RECORD_NOT_FOUND: ErrorCode = 7;

// User (100..199)
pub const WRONG_USER_PASSWORD: ErrorCode = 100;
pub const NEXT_ID_EXPIRED: ErrorCode = 101;
pub const ACCOUNT_BLOCKED: ErrorCode = 102;
pub const ACCESS_DENIED: ErrorCode = 103;

static ERROR_MESSAGES: LazyLock<HashMap<ErrorCode, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(PARSE_ERROR, "Parse error");
    m.insert(CONTROLLER_NOT_FOUND, "Controller not found");
    m.insert(METHOD_NOT_FOUND, "Method not found");
    m.insert(PARAMETER_NOT_FOUND, "Parameter not found");
    m.insert(INTERNAL_SERVER_ERROR, "Internal server error");
    m.insert(INVALID_PARAMETER, "Invalid parameter");
    m.insert(RECORD_NOT_FOUND, "Record not found");

    m.insert(WRONG_USER_PASSWORD, "Wrong user password");
    m.insert(NEXT_ID_EXPIRED, "Next id expired");
    m.insert(ACCOUNT_BLOCKED, "Account blocked");
    m.insert(ACCESS_DENIED, "Access denied");
    m
});

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
            message: (*ERROR_MESSAGES.get(&code).unwrap()).to_string(),
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

pub fn make_error_data(code: ErrorCode, data: &str) -> Box<dyn error::Error> {
    Box::new(Error::new(code, Some(data.to_string())))
}
