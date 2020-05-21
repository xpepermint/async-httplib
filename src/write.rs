use async_std::prelude::*;
use async_std::io::{Read, Write};
use std::io::{Error, ErrorKind};
use crate::{relay_exact};

pub async fn write_slice<O>(output: &mut O, data: &[u8]) -> Result<usize, Error>
    where
    O: Write + Unpin,
{
    output.write(data).await
}

pub async fn flush_write<O>(output: &mut O) -> Result<(), Error>
    where
    O: Write + Unpin,
{
    output.flush().await
}

pub async fn write_exact<O, I>(output: &mut O, input: &mut I, length: usize) -> Result<usize, Error>
    where
    O: Write + Unpin,
    I: Read + Unpin,
{
    relay_exact(input, output, length).await
}

pub async fn write_all<O, I>(output: &mut O, input: &mut I, limit: Option<usize>) -> Result<usize, Error>
    where
    O: Write + Unpin,
    I: Read + Unpin,
{
    let mut total = 0; // all written bytes
    let mut length = 0; // data written bytes
    
    loop {
        let mut bytes = vec![0u8; 1024];
        let size = input.read(&mut bytes).await?;
        bytes = bytes[0..size].to_vec();
        length += size;

        if size == 0 {
            break;
        } else if limit.is_some() && length > limit.unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while writing.", limit.unwrap())));
        }

        total += output.write(&bytes).await?;
        output.flush().await?;
    }

    Ok(total)
}

pub async fn write_chunks<O, I>(output: &mut O, input: &mut I, limits: (Option<usize>, Option<usize>)) -> Result<usize, Error>
    where
    O: Write + Unpin,
    I: Read + Unpin,
{
    let (chunklimit, datalimit) = limits;
    let chunksize = match chunklimit {
        Some(chunksize) => chunksize,
        None => 1024,
    };
    let mut total = 0; // all written bytes
    let mut length = 0; // data written bytes
    
    loop {
        let mut bytes = vec![0u8; chunksize];
        let size = input.read(&mut bytes).await?;
        bytes = bytes[0..size].to_vec();
        length += size;

        if datalimit.is_some() && length > datalimit.unwrap() {
            return Err(Error::new(ErrorKind::InvalidData, format!("The operation hit the limit of {} bytes while writing chunked HTTP body.", datalimit.unwrap())));
        }

        total += output.write(format!("{:x}\r\n", size).as_bytes()).await?;
        total += output.write(&bytes).await?;
        total += output.write(b"\r\n").await?;
        output.flush().await?;

        if size == 0 {
            break;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn writes_exact() {
        let mut output = Vec::new();
        let size = write_exact(&mut output, &mut "0123456789".as_bytes(), 5).await.unwrap();
        assert_eq!(size, 5);
        assert_eq!(output, b"01234");
    }

    #[async_std::test]
    async fn writes_all() {
        let mut output = Vec::new();
        let size = write_all(&mut output, &mut "0123456789".as_bytes(), None).await.unwrap();
        assert_eq!(size, 10);
        assert_eq!(output, b"0123456789");
        let mut output = Vec::new();
        let exceeded = write_all(&mut output, &mut "012".as_bytes(), Some(2)).await;
        assert!(exceeded.is_err());
    }

    #[async_std::test]
    async fn writes_chunks() {
        let mut output = Vec::new();
        let size = write_chunks(&mut output, &mut "0123456789".as_bytes(), (Some(3), None)).await.unwrap();
        assert_eq!(size, 35);
        assert_eq!(output, "3\r\n012\r\n3\r\n345\r\n3\r\n678\r\n1\r\n9\r\n0\r\n\r\n".as_bytes());
        let mut output = Vec::new();
        let exceeded = write_chunks(&mut output, &mut "0123456789".as_bytes(), (Some(3), Some(4))).await;
        assert!(exceeded.is_err());
    }
}
