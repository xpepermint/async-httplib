
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidMethod,
    InvalidVersion,
    InvalidStatus,
    InvalidInput,
    UnableToRead,
    UnableToWrite,
    LimitExceeded,
}
