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

pub async fn write_chunks<I, O>(input: &mut I, output: &mut O, limits: (Option<usize>, Option<usize>)) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    let (chunklimit, datalimit) = limits;
    let chunksize = match chunklimit {
        Some(chunksize) => chunksize,
        None => 1024,
    };
    let mut total = 0; // all written bytes
    let mut length = 0; // data written bytes
    
    loop {
        println!("===========> total:{} + chunksize:{} > datalimit:{:?}", total, chunksize, datalimit);

        let chunksize = match datalimit {
            Some(datalimit) => match length + chunksize > datalimit {
                true => datalimit - length,
                false => chunksize,
            },
            None => chunksize,
        };

        let mut bytes = vec![0u8; chunksize];
        let size = match input.read(&mut bytes).await {
            Ok(size) => size,
            Err(_) => return Err(Error::UnableToRead),
        };
        bytes = bytes[0..size].to_vec();
        length += size;

        println!("===========> size:{}", size);

        total += write_all(output, format!("{:x}\r\n", size).as_bytes()).await?;
        total += write_all(output, &bytes).await?;
        total += write_all(output, b"\r\n").await?;
        flush_write(output).await?;

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
        let size = write_exact(&mut "0123456789".as_bytes(), &mut output, 5).await.unwrap();
        assert_eq!(size, 5);
        assert_eq!(output, b"01234");
    }

    #[async_std::test]
    async fn writes_chunks() {
        let mut output = Vec::new();
        let size = write_chunks(&mut "0123456789".as_bytes(), &mut output, (Some(3), None)).await.unwrap();
        assert_eq!(size, 35);
        assert_eq!(output, "3\r\n012\r\n3\r\n345\r\n3\r\n678\r\n1\r\n9\r\n0\r\n\r\n".as_bytes());
        let mut output = Vec::new();
        let size = write_chunks(&mut "0123456789".as_bytes(), &mut output, (Some(3), Some(4))).await.unwrap();
        assert_eq!(size, 19);
        assert_eq!(output, "3\r\n012\r\n1\r\n3\r\n0\r\n\r\n".as_bytes());
    }
}
