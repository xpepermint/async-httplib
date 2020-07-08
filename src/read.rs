use std::io::{Error, ErrorKind};
use async_std::prelude::*;
use async_std::io::{Read};

pub async fn read_first_line<I>(input: &mut I, data: (&mut Vec<u8>, &mut Vec<u8>, &mut Vec<u8>), limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let mut part = 0;
    let mut length = 0;
    let mut stage = 0; // 0..data, 1..\r, 2..\n

    loop {
        let mut bytes = [0u8];
        let size = input.read(&mut bytes).await?;
        length += size;

        if size == 0 {
            break;
        } else if limit.is_some() && limit.unwrap() < length { // method + url + version = 2065
            return Err(Error::new(ErrorKind::InvalidData, "The operation hit the limit of {} bytes while reading the HTTP first line."));
        } else if bytes[0] == 32 { // space
            part += 1;
            continue;
        } else if bytes[0] == 13 { // \r
            stage = 1;
            continue;
        } else if bytes[0] == 10 { // \n
            if stage == 1 {
                break;
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "The data is not a valid HTTP first line."));
            }
        }

        match part {
            0 => data.0.push(bytes[0]),
            1 => data.1.push(bytes[0]),
            _ => data.2.push(bytes[0]),
        };
    }

    Ok(length)
}

pub async fn read_header_line<I>(input: &mut I, data: (&mut Vec<u8>, &mut Vec<u8>), limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let mut length = 0;
    let mut stage = 0; // 0..name, 1..:, 2..space, 3..value, 4..\r, 5..\n

    loop {
        let mut bytes = [0u8];
        let size = input.read(&mut bytes).await?;
        length += size;

        if size == 0 {
            break;
        } else if limit.is_some() && limit.unwrap() < length {
            return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while reading the HTTP header line.", limit.unwrap())));
        } else if stage == 0 && bytes[0] == 58 { // first :
            stage = 1;
            continue;
        } else if stage == 1 && bytes[0] == 32 { // first space
            stage = 2;
            continue;
        } else if bytes[0] == 13 { // first/second \r
            if stage == 0 || stage == 2 {
                stage = 3;
                continue;
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "The data is not a valid HTTP header line."));
            }
        } else if bytes[0] == 10 { // first/second \n
            if stage == 3 {
                break;
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "The data is not a valid HTTP header line."));
            }
        }

        if stage == 0 {
            data.0.push(bytes[0]);
        } else if stage == 2 {
            data.1.push(bytes[0]);
        }
    }

    Ok(length)
}

pub async fn read_exact<I>(input: &mut I, data: &mut Vec<u8>, length: usize) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let mut bytes = vec![0u8; length];
    input.read_exact(&mut bytes).await?;
    data.append(&mut bytes);

    Ok(length)
}

pub async fn read_chunk_line<I>(input: &mut I, data: (&mut Vec<u8>, &mut Vec<u8>), limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let mut length = 0;
    let mut stage = 0; // 0..number, 1..extension 2..\r, 3=\n

    loop {
        let mut bytes = [0u8];
        let size = input.read(&mut bytes).await?;
        length += size;

        if size == 0 { // end of data
            break;
        } else if limit.is_some() && limit.unwrap() < length {
            return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while reading the HTTP body chunk line.", limit.unwrap())));
        } else if stage == 0 && bytes[0] == 59 { // char ;
            stage = 1;
            continue;
        } else if bytes[0] == 13 { // char \r
            if stage == 0 || stage == 1 {
                stage = 2;
                continue;
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "The data is not a valid HTTP chunk line."));
            }
        } else if bytes[0] == 10 { // char \n
            break;
        }
        match stage {
            0 => data.0.push(bytes[0]),
            1 => data.1.push(bytes[0]),
            _ => (),
        };
    }

    Ok(length)
}

pub async fn read_chunks<I>(input: &mut I, data: &mut Vec<u8>, limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let mut length = 0;

    loop {
        let limit = match limit {
            Some(limit) => Some(limit - length),
            None => None,
        };
        let mut buff = Vec::new();
        let size = read_chunk(input, &mut buff, limit).await?;
        length += size;

        if size == 0 || buff.len() == 0 {
            break; // last chunk
        } else {
            data.append(&mut buff);
        }
    }

    Ok(length)
}

pub async fn read_chunk<I>(input: &mut I, data: &mut Vec<u8>, limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
{
    let (mut length, mut ext) = (vec![], vec![]);
    let mut size = read_chunk_line(input, (&mut length, &mut ext), limit).await?;
    let length = match String::from_utf8(length) {
        Ok(length) => match i64::from_str_radix(&length, 16) {
            Ok(length) => length as usize,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        },
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    };

    if limit.is_some() && length > limit.unwrap() {
        return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while reading the HTTP body chunk.", limit.unwrap())));
    } else {
        size += read_exact(input, data, length).await?;
        size += read_exact(input, &mut Vec::new(), 2).await?;
    }

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn reads_first_line() {
        let (mut a, mut b, mut c) = (vec![], vec![], vec![]);
        let size = read_first_line(&mut "OPTIONS /path HTTP/1.1\r\n".as_bytes(), (&mut a, &mut b, &mut c), None).await.unwrap();
        assert_eq!(size, 24);
        assert_eq!(a, b"OPTIONS");
        assert_eq!(b, b"/path");
        assert_eq!(c, b"HTTP/1.1");
        let (mut a, mut b, mut c) = (vec![], vec![], vec![]);
        let exceeded = read_first_line(&mut "OPTI\r\n".as_bytes(), (&mut a, &mut b, &mut c), Some(1)).await;
        assert!(exceeded.is_err());
    }

    #[async_std::test]
    async fn reads_header() {
        let (mut name, mut value) = (vec![], vec![]);
        let size = read_header_line(&mut "Foo: foo\r\nBar: bar\r\n".as_bytes(), (&mut name, &mut value), None).await.unwrap();
        assert_eq!(size, 10);
        assert_eq!(name, b"Foo");
        assert_eq!(value, b"foo");
        let (mut name, mut value) = (vec![], vec![]);
        let size = read_header_line(&mut "\r\n".as_bytes(), (&mut name, &mut value), None).await.unwrap();
        assert_eq!(size, 2);
        assert_eq!(name, b"");
        assert_eq!(value, b"");
        let exceeded = read_header_line(&mut "Foo".as_bytes(), (&mut name, &mut value), Some(1)).await;
        assert!(exceeded.is_err());
    }

    #[async_std::test]
    async fn reads_exact() {
        let mut output = Vec::new();
        read_exact(&mut "0123456789".as_bytes(), &mut output, 5).await.unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "01234");
    }

    #[async_std::test]
    async fn reads_chunk_line() {
        let (mut number, mut ext) = (vec![], vec![]);
        let size = read_chunk_line(&mut "6;ex;ex\r\n".as_bytes(), (&mut number, &mut ext), None).await.unwrap();
        assert_eq!(size, 9);
        assert_eq!(String::from_utf8(number).unwrap(), "6");
        assert_eq!(String::from_utf8(ext).unwrap(), "ex;ex");
        let (mut number, mut ext) = (vec![], vec![]);
        let exceeded = read_chunk_line(&mut "6\r\n".as_bytes(), (&mut number, &mut ext), Some(1)).await;
        assert!(exceeded.is_err());
    }

    #[async_std::test]
    async fn reads_chunks() {
        let mut output = Vec::new();
        let size = read_chunks(&mut "6\r\nHello \r\n6;ex=fo\r\nWorld!\r\n0\r\n\r\nTrail: er\r\n\r\n".as_bytes(), &mut output, None).await.unwrap(); // with extension `ex=fo` and trailer `Trail: er`
        assert_eq!(size, 33);
        assert_eq!(String::from_utf8(output).unwrap(), "Hello World!");
        let mut output = Vec::new();
        let exceeded = read_chunks(&mut "6\r\nHello 0\r\n\r\n".as_bytes(), &mut output, Some(1)).await;
        assert!(exceeded.is_err());
    }
}
