#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Error {
    InvalidMethod,
    InvalidVersion,
    InvalidStatus,
    InvalidInput,
    UnableToRead,
    UnableToWrite,
    LimitExceeded,
}
