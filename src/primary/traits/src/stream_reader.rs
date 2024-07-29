use std::io::Cursor;
use async_trait::async_trait;
use byteorder::{LittleEndian, ReadBytesExt};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt};

use crate::errors::FieldError;

#[async_trait]
pub trait StreamReader {
    async fn read_from<R>(stream: &mut R, dependencies: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send;
}

#[async_trait]
impl StreamReader for u8 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send,
    {
        stream.read_u8().await.map_err(|e| FieldError::CannotRead(e, "u8".to_string()))
    }
}

#[async_trait]
impl StreamReader for u16 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_u16_le().await.map_err(|e| FieldError::CannotRead(e, "u16".to_string()))
    }

}

#[async_trait]
impl StreamReader for u32 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_u32_le().await.map_err(|e| FieldError::CannotRead(e, "u32".to_string()))
    }
}

#[async_trait]
impl StreamReader for u64 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_u64_le().await.map_err(|e| FieldError::CannotRead(e, "u64".to_string()))
    }
}

#[async_trait]
impl StreamReader for i8 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_i8().await.map_err(|e| FieldError::CannotRead(e, "i8".to_string()))
    }

}

#[async_trait]
impl StreamReader for i16 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_i16_le().await.map_err(|e| FieldError::CannotRead(e, "i16".to_string()))
    }

}

#[async_trait]
impl StreamReader for i32 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_i32_le().await.map_err(|e| FieldError::CannotRead(e, "i32".to_string()))
    }
}

#[async_trait]
impl StreamReader for i64 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_i64_le().await.map_err(|e| FieldError::CannotRead(e, "i64".to_string()))
    }
}

#[async_trait]
impl StreamReader for f32 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_f32_le().await.map_err(|e| FieldError::CannotRead(e, "f32".to_string()))
    }
}

#[async_trait]
impl StreamReader for f64 {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        stream.read_f64_le().await.map_err(|e| FieldError::CannotRead(e, "f64".to_string()))
    }
}

#[async_trait]
impl StreamReader for String {
    async fn read_from<R>(stream: &mut R, dependencies: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        let mut cursor = Cursor::new(dependencies.to_vec());

        let size = match dependencies.len() {
            1 => ReadBytesExt::read_u8(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("String u8 size")))? as usize,
            2 => ReadBytesExt::read_u16::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("String u16 size")))? as usize,
            4 => ReadBytesExt::read_u32::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("String u32 size")))? as usize,
            _ => 0,
        };

        let buffer = if size > 0 {
            let mut buffer = vec![0u8; size];
            stream.read_exact(&mut buffer).await
                .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
            buffer
        } else {
            let mut buffer = vec![];
            stream.read_until(0, &mut buffer).await
                .map_err(|e| FieldError::CannotRead(e, "String".to_string()))?;
            buffer
        };

        let string = String::from_utf8(buffer)
            .map_err(|e| FieldError::InvalidString(e, "String".to_string()))?;

        Ok(string.trim_end_matches(char::from(0)).to_string())
    }
}

#[async_trait]
impl<const N: usize> StreamReader for [u8; N] {
    async fn read_from<R>(stream: &mut R, _: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send
    {
        let mut internal_buf = [0; N];
        stream.read_exact(&mut internal_buf).await
            .map_err(|e| FieldError::CannotRead(e, "[u8; N]".to_string()))?;
        Ok(internal_buf)
    }
}

#[async_trait]
impl<T: StreamReader + Clone + Send> StreamReader for Vec<T> {
    async fn read_from<R>(stream: &mut R, dependencies: &mut Vec<u8>) -> Result<Self, FieldError>
        where
            Self: Sized,
            R: AsyncBufRead + Unpin + Send,
    {
        let mut cursor = Cursor::new(dependencies.to_vec());
        let size = match dependencies.len() {
            1 => ReadBytesExt::read_u8(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("Vec<T> u8 size")))? as usize,
            2 => ReadBytesExt::read_u16::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("Vec<T> u16 size")))? as usize,
            _ => ReadBytesExt::read_u32::<LittleEndian>(&mut cursor)
                .map_err(|e| FieldError::CannotRead(e, format!("Vec<T> u32 size")))? as usize,
        };

        let mut buffer: Vec<T> = vec![];

        for _ in 0..size {
            buffer.push(T::read_from(stream, dependencies).await?);
        }

        Ok(buffer)
    }
}