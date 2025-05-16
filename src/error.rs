use std::fmt::{self, write};
use std::error::Error;

#[derive(Debug)]
pub enum HyperLogLogError { 
    MisMatchedPrecision(u32, u32),
    MergeFailed(String),
    PrecisionBelowThreshold,
    PrecisionTooLarge
}

impl fmt::Display for HyperLogLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HyperLogLogError::MisMatchedPrecision(expected, actual) => {
                write!(f, "Precision mismatch: expected {}, found {}", expected, actual)
            }
            HyperLogLogError::MergeFailed(msg) => {
                write!(f, "Merge failed {}", msg)
            }
            HyperLogLogError::PrecisionBelowThreshold => {
                write!(f, "Precision p must be at least 4")
            }
            HyperLogLogError::PrecisionTooLarge => {
                write!(f, "Precision too large, reduce p")
            }

        }
    }
}

impl Error for HyperLogLogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

