use std::fmt::{self, Display};
use std::str::FromStr;
use crate::Error;

/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {

    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.1) for more details.
    pub fn is_safe(&self) -> bool {
        match self {
            Method::Get | Method::Head | Method::Options | Method::Trace => true,
            _ => false,
        }
    }

    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.2) for more details.
    pub fn is_idempotent(&self) -> bool {
        match self {
            Method::Get | Method::Head | Method::Options | Method::Trace | Method::Put | Method::Delete => true,
            _ => false,
        }
    }

    /// See [the spec](https://tools.ietf.org/html/rfc7231#section-4.2.3) for more details.
    pub fn is_cacheable(&self) -> bool {
        match self {
            Method::Get | Method::Head => true,
            _ => false,
        }
    }
}

impl Display for Method {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Trace => write!(f, "TRACE"),
            Self::Patch => write!(f, "PATCH"),
        }
    }
}

impl FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(Error::InvalidMethod),
        }
    }
}

impl<'a> std::convert::TryFrom<&[u8]> for Method {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(txt) => Self::from_str(&txt),
            Err(_) => Err(Error::InvalidStatus),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn implements_from_str() {
        let method = Method::from_str("POST").unwrap();
        assert_eq!(method, Method::Post);
    }

    #[test]
    fn implements_try_from() {
        let method = Method::try_from("POST".as_bytes()).unwrap();
        assert_eq!(method, Method::Post);
    }

    #[test]
    fn implements_to_string() {
        let method = Method::from_str("POST").unwrap();
        assert_eq!(method.to_string(), "POST");
    }
}
