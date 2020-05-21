use std::io::{Error, ErrorKind};
use async_std::prelude::*;
use async_std::io::{Read, Write};
use crate::{read_chunk_line, write_slice, flush_write};

pub async fn relay_exact<I, O>(input: &mut I, output: &mut O, length: usize) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    if length == 0 {
        return Ok(0);
    }

    let bufsize = 1024;
    let mut total = 0;
    
    loop {
        let bufsize = match length - total < bufsize {
            true => length - total, // do not read more than necessary
            false => bufsize,
        };

        let mut bytes = vec![0u8; bufsize];
        let size = input.read(&mut bytes).await?;
        total += size;

        write_slice(output, &bytes).await?;
        flush_write(output).await?;

        if total == length {
            break;
        }
    }

    Ok(total)
}

pub async fn relay_chunks<I, O>(input: &mut I, output: &mut O, limits: (Option<usize>, Option<usize>)) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    let (chunklimit, datalimit) = limits;
    let mut length = 0;
    let mut total = 0; // actual data size

    loop {
        let (mut size, mut ext) = (vec![], vec![]);
        read_chunk_line(input, (&mut size, &mut ext), chunklimit).await?;

        length += write_slice(output, &size).await?;
        if !ext.is_empty() {
            length += write_slice(output, b";").await?;
            length += write_slice(output, &ext).await?;
        }
        length += write_slice(output, b"\r\n").await?;

        let size = match String::from_utf8(size) {
            Ok(length) => match i64::from_str_radix(&length, 16) {
                Ok(length) => length as usize,
                Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
            },
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        };

        if size == 0 {
            length += relay_exact(input, output, 2).await?;
            break; // last chunk
        } else if datalimit.is_some() && total + size > datalimit.unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while relaying chunked HTTP body.", datalimit.unwrap())));
        } else {
            total += size;
            length += relay_exact(input, output, size).await?;
            length += relay_exact(input, output, 2).await?;
        }
    }

    Ok(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn relays_exact() {
        let mut output = Vec::new();
        let size = relay_exact(&mut "0123456789".as_bytes(), &mut output, 5).await.unwrap();
        assert_eq!(size, 5);
        assert_eq!(output, b"01234");
    }

    #[async_std::test]
    async fn relays_chunks() {
        let mut output = Vec::new();
        let size = relay_chunks(&mut "6\r\nHello \r\n6;ex;ey\r\nWorld!\r\n0\r\n\r\nFoo: bar\r\n\r\n".as_bytes(), &mut output, (None, None)).await.unwrap();
        assert_eq!(size, 33);
        assert_eq!(output, "6\r\nHello \r\n6;ex;ey\r\nWorld!\r\n0\r\n\r\n".as_bytes());
        let mut output = Vec::new();
        let exceeds = relay_chunks(&mut "3\r\nHel\r\n0;ex;".as_bytes(), &mut output, (None, Some(2))).await;
        assert!(exceeds.is_err());
    }
}
