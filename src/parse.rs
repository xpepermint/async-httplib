use std::io::{Error, ErrorKind};
use std::convert::TryFrom;
use crate::{Method, Status, Version};

pub fn parse_method(data: Vec<u8>) -> Result<Method, Error> {
    let data: &[u8] = &data;
    Method::try_from(data)
}

pub fn parse_uri(data: Vec<u8>) -> Result<String, Error> {
    match String::from_utf8(data) {
        Ok(uri) => Ok(uri),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string()))
    }
}

pub fn parse_status(data: Vec<u8>) -> Result<Status, Error> {
    let data: &[u8] = &data;
    Status::try_from(data)
}

pub fn parse_version(data: Vec<u8>) -> Result<Version, Error> {
    let data: &[u8] = &data;
    Version::try_from(data)
}
