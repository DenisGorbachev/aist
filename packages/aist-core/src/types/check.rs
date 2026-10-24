use std::error::Error as StdError;
use thiserror::Error;

#[derive(Debug)]
pub struct Check {
    pub name: String,
    pub result: Result<(), CheckError>,
}

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("predicate is false: {predicate}")]
    PredicateInvalid { predicate: String },
    #[error("expected {expected}, found {actual}")]
    EqualityInvalid { expected: String, actual: String },
    #[error("operation failed")]
    OperationFailed { source: Box<dyn StdError> },
    #[error("unverified universal obligation: {predicate}")]
    ProofNotFound { predicate: String },
    #[error("contract panicked: {message}")]
    PanickedFailed { message: String },
    #[error("contract produced no checks")]
    ChecksNotFound {},
}
