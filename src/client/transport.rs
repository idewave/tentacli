use std::sync::Arc;
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UdpSocket;

#[async_trait]
pub trait TransportRead: Send + Sync {
    async fn read(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize>;
}

#[async_trait]
pub trait TransportWrite: Send + Sync {
    async fn write(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize>;
}

pub struct TcpRead(pub OwnedReadHalf);
#[async_trait]
impl TransportRead for TcpRead {
    async fn read(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        self.0.read(buffer).await.map_err(Into::into)
    }
}

pub struct TcpWrite(pub OwnedWriteHalf);
#[async_trait]
impl TransportWrite for TcpWrite {
    async fn write(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        self.0.write(buffer).await.map_err(Into::into)
    }
}

pub struct UdpRead(pub Arc<UdpSocket>);
#[async_trait]
impl TransportRead for UdpRead {
    async fn read(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        self.0.recv(buffer).await.map_err(Into::into)
    }
}

pub struct UdpWrite(pub Arc<UdpSocket>);
#[async_trait]
impl TransportWrite for UdpWrite {
    async fn write(&mut self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        self.0.send(buffer).await.map_err(Into::into)
    }
}

#[allow(dead_code)]
pub enum Protocol {
    TCP,
    UDP,
}