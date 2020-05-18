use std::fmt::{self, Display};
use std::str::FromStr;
use crate::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Version {
    Http0_9,
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
}

impl Display for Version {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http0_9 => write!(f, "HTTP/0.9"),
            Self::Http1_0 => write!(f, "HTTP/1.0"),
            Self::Http1_1 => write!(f, "HTTP/1.1"),
            Self::Http2_0 => write!(f, "HTTP/2"),
            Self::Http3_0 => write!(f, "HTTP/3"),
        }
    }
}

impl FromStr for Version {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/0.9" => Ok(Self::Http0_9),
            "0.9" => Ok(Self::Http0_9),
            "HTTP/1.0" => Ok(Self::Http1_0),
            "1.0" => Ok(Self::Http1_0),
            "HTTP/1.1" => Ok(Self::Http1_1),
            "1.1" => Ok(Self::Http1_1),
            "1" => Ok(Self::Http1_1),
            "HTTP/2" => Ok(Self::Http2_0),
            "2.0" => Ok(Self::Http2_0),
            "2" => Ok(Self::Http2_0),
            "HTTP/3" => Ok(Self::Http3_0),
            "3.0" => Ok(Self::Http3_0),
            "3" => Ok(Self::Http3_0),
            _ => Err(Error::InvalidVersion),
        }
    }
}

impl<'a> std::convert::TryFrom<&[u8]> for Version {
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
        let version = Version::from_str("1.1").unwrap();
        assert_eq!(version, Version::Http1_1);
    }

    #[test]
    fn implements_try_from() {
        let version = Version::try_from("1.1".as_bytes()).unwrap();
        assert_eq!(version, Version::Http1_1);
    }

    #[test]
    fn implements_to_string() {
        let version = Version::from_str("2.0").unwrap();
        assert_eq!(version.to_string(), "HTTP/2");
    }
}
