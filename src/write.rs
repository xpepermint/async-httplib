use async_std::prelude::*;
use async_std::io::{Read, Write};
use crate::{Error, relay_exact};

pub async fn write_all<O>(output: &mut O, data: &[u8]) -> Result<usize, Error>
    where
    O: Write + Unpin,
{
    match output.write(data).await {
        Ok(size) => Ok(size),
        Err(_) => Err(Error::UnableToWrite),
    }
}

pub async fn write_line<O>(output: &mut O, data: &[u8]) -> Result<usize, Error>
    where
    O: Write + Unpin,
{
    write_all(output, data).await?;
    write_all(output, b"\r\n").await
}

pub async fn flush_write<O>(output: &mut O) -> Result<(), Error>
    where
    O: Write + Unpin,
{
    match output.flush().await {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::UnableToWrite),
    }
}

pub async fn write_exact<I, O>(input: &mut I, output: &mut O, length: usize) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    relay_exact(input, output, length).await
}

pub async fn write_chunks<I, O>(input: &mut I, output: &mut O, lengths: (usize, usize)) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    let (chunklen, datalen) = lengths;
    let mut total = 0;
    
    loop {
        let chunksize = match datalen - total < chunklen {
            true => datalen - total, // do not read more than necessary
            false => chunklen,
        };

        if chunksize == 0 {
            write_all(output, b"0\r\n").await?;
            write_all(output, b"\r\n").await?;
            break;
        }

        let mut bytes = vec![0u8; chunksize];
        let size = match input.read(&mut bytes).await {
            Ok(size) => size,
            Err(_) => return Err(Error::UnableToRead),
        };
        total += size;

        write_all(output, format!("{:x}\r\n", chunksize).as_bytes()).await?;
        write_all(output, &bytes).await?;
        write_all(output, b"\r\n").await?;
        flush_write(output).await?;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn writes_exact() {
        let mut output = Vec::new();
        let size = write_exact(&mut "0123456789".as_bytes(), &mut output, 5).await.unwrap();
        assert_eq!(size, 5);
        assert_eq!(output, b"01234");
    }

    #[async_std::test]
    async fn writes_chunks() {
        let mut output = Vec::new();
        let size = write_chunks(&mut "0123456789".as_bytes(), &mut output, (3, 10)).await.unwrap();
        assert_eq!(size, 10);
        assert_eq!(output, "3\r\n012\r\n3\r\n345\r\n3\r\n678\r\n1\r\n9\r\n0\r\n\r\n".as_bytes());
    }
}
