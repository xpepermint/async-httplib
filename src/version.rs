use std::fmt::{self, Display};
use std::cmp::Ordering;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub enum Version {
    Http0_9 = 9,
    Http1_0 = 10,
    Http1_1 = 11,
    Http2_0 = 20,
    Http3_0 = 30,
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
    type Err = Error;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
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
            v => Err(Error::new(ErrorKind::InvalidInput, format!("The version `{}` is invalid.", v))),
        }
    }
}

impl<'a> std::convert::TryFrom<&[u8]> for Version {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(txt) => Self::from_str(&txt),
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e.to_string())),
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        (*self as usize) == (*other as usize)
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

    #[test]
    fn implements_ordering() {
        assert!(Version::Http1_1 > Version::Http0_9);
        assert!(Version::Http0_9 < Version::Http1_0);
        assert!(Version::Http1_0 == Version::Http1_0);
    }
}
