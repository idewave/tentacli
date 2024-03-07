use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

use crate::errors::FieldError;

#[async_trait]
pub trait StreamReader {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized;
}

#[async_trait]
impl StreamReader for u8 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_u8().await.map_err(|e| FieldError::CannotRead(e, "u8".to_string()))
    }
}

#[async_trait]
impl StreamReader for u16 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_u16_le().await.map_err(|e| FieldError::CannotRead(e, "u16".to_string()))
    }

}

#[async_trait]
impl StreamReader for u32 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_u32_le().await.map_err(|e| FieldError::CannotRead(e, "u32".to_string()))
    }
}

#[async_trait]
impl StreamReader for u64 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_u64_le().await.map_err(|e| FieldError::CannotRead(e, "u64".to_string()))
    }
}

#[async_trait]
impl StreamReader for i8 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_i8().await.map_err(|e| FieldError::CannotRead(e, "i8".to_string()))
    }

}

#[async_trait]
impl StreamReader for i16 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_i16_le().await.map_err(|e| FieldError::CannotRead(e, "i16".to_string()))
    }

}

#[async_trait]
impl StreamReader for i32 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_i32_le().await.map_err(|e| FieldError::CannotRead(e, "i32".to_string()))
    }
}

#[async_trait]
impl StreamReader for i64 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_i64_le().await.map_err(|e| FieldError::CannotRead(e, "i64".to_string()))
    }
}

#[async_trait]
impl StreamReader for f32 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_f32_le().await.map_err(|e| FieldError::CannotRead(e, "f32".to_string()))
    }
}

#[async_trait]
impl StreamReader for f64 {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        stream.read_f64_le().await.map_err(|e| FieldError::CannotRead(e, "f64".to_string()))
    }
}

#[async_trait]
impl StreamReader for String {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        let mut internal_buf = vec![];
        stream.read_until(0, &mut internal_buf).await
            .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
        String::from_utf8(
            internal_buf[..internal_buf.len()].to_vec()
        ).map_err(|e| FieldError::InvalidString(e, "String".to_string()))
    }
}

#[async_trait]
impl<const N: usize> StreamReader for [u8; N] {
    async fn read_from(stream: &mut BufReader<TcpStream>) -> Result<Self, FieldError>
        where Self: Sized
    {
        let mut internal_buf = [0; N];
        stream.read_exact(&mut internal_buf).await
            .map_err(|e| FieldError::CannotRead(e, "[u8; N]".to_string()))?;
        Ok(internal_buf)
    }
}