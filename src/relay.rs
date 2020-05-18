use async_std::prelude::*;
use async_std::io::{Read, Write};
use crate::{Error, write_all, flush_write};
use crate::utils::{has_sequence};

// TODO: Do not read more then `length`!
// TODO: Respect trailers!
pub async fn relay_chunks<I, O>(input: &mut I, output: &mut O, limit: Option<usize>) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    let mut buffer: Vec<u8> = Vec::new();
    let mut count = 0;
    loop {
        if limit.is_some() && count >= limit.unwrap() {
            return Err(Error::LimitExceeded);
        }

        let mut bytes = [0u8; 1024];
        let size = match input.read(&mut bytes).await {
            Ok(size) => size,
            Err(_) => return Err(Error::UnableToRead),
        };
        let mut bytes = &mut bytes[0..size].to_vec();
        count += size;

        write_all(output, &bytes).await?;
        flush_write(output).await?;

        buffer.append(&mut bytes);
        buffer = (&buffer[buffer.len()-5..]).to_vec();
        if has_sequence(&buffer, &[48, 13, 10, 13, 10]) { // last chunk
            break;
        }
        buffer = (&buffer[buffer.len()-5..]).to_vec();
    }

    Ok(count)
}

// TODO: Do not read more then `length`!
pub async fn relay_exact<I, O>(input: &mut I, output: &mut O, length: usize) -> Result<usize, Error>
    where
    I: Read + Unpin,
    O: Write + Unpin,
{
    if length == 0 {
        return Ok(0);
    }

    let mut count = 0;
    loop {
        let mut bytes = [0u8; 1024];
        let size = match input.read(&mut bytes).await {
            Ok(size) => size,
            Err(_) => return Err(Error::UnableToRead),
        };
        let bytes = &mut bytes[0..size].to_vec();
        count += size;

        write_all(output, &bytes).await?;
        flush_write(output).await?;

        if size == 0 || count == length {
            break;
        } else if count > length {
            return Err(Error::LimitExceeded);
        }
    }

    Ok(count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn relays_chunks() {
        let mut input = "6\r\nHello \r\n6\r\nWorld!\r\n0\r\n\r\nFoo: bar\r\n\r\n".as_bytes();
        // let mut output = Vec::new();
        // relay_chunks(&mut input, &mut output, None).await.unwrap();
        // assert_eq!(output, b"Hello World!");
    }
}
