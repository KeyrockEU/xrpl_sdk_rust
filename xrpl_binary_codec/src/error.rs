use crate::alloc::string::String;
use core::fmt;

/// Result type for binary codec operations.
pub type Result<T, E = BinaryCodecError> = core::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryCodecError {
    OutOfRange(String),
    FieldOrder(String),
    InvalidField(String),
    MissingField(String),
    InvalidLength(String),
    InsufficientBytes(String),
}

#[cfg(feature = "std")]
impl std::error::Error for BinaryCodecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for BinaryCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange(s) => write!(f, "Value not within the required range: {}", s),
            Self::FieldOrder(s) => write!(f, "Field order is wrong: {}", s),
            Self::InvalidField(s) => write!(f, "Invalid field: {}", s),
            Self::MissingField(s) => write!(f, "Missing field: {}", s),
            Self::InvalidLength(s) => write!(f, "Invalid length: {}", s),
            Self::InsufficientBytes(s) => write!(f, "Insufficient bytes to decode: {}", s),
        }
    }
}

