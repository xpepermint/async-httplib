use async_std::prelude::*;
use async_std::io::{Write};
use crate::{Error};

pub async fn write_all<S>(output: &mut S, data: &[u8]) -> Result<usize, Error>
    where
    S: Write + Unpin,
{
    match output.write(data).await {
        Ok(size) => Ok(size),
        Err(_) => Err(Error::UnableToWrite),
    }
}

pub async fn write_line<S>(output: &mut S, data: &[u8]) -> Result<usize, Error>
    where
    S: Write + Unpin,
{
    write_all(output, data).await?;
    write_all(output, b"\r\n").await
}

pub async fn flush_write<S>(output: &mut S) -> Result<(), Error>
    where
    S: Write + Unpin,
{
    match output.flush().await {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::UnableToWrite),
    }
}
