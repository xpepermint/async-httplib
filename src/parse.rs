use std::io::Error;
use std::convert::TryFrom;
use crate::{Method, Status, Version};

pub fn parse_method(data: Vec<u8>) -> Result<Method, Error> {
    let data: &[u8] = &data;
    Method::try_from(data)
}

pub fn parse_status(data: Vec<u8>) -> Result<Status, Error> {
    let data: &[u8] = &data;
    Status::try_from(data)
}

pub fn parse_version(data: Vec<u8>) -> Result<Version, Error> {
    let data: &[u8] = &data;
    Version::try_from(data)
}
