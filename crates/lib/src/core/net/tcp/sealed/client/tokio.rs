use std::{net::SocketAddr, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    time::timeout as tokio_timeout,
};

use crate::{
    error::{
        NetworkError,
        Report,
        Result,
        _metadata::{NetworkInterface, NetworkProtocol},
    },
    settings::Timeout,
};

#[derive(Debug)]
pub(super) struct AsyncTokioTcpClient {
    read_stream: Arc<Mutex<OwnedReadHalf>>,
    read_timeout: Duration,
    write_stream: Arc<Mutex<OwnedWriteHalf>>,
    write_timeout: Duration,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncTokioTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        let (orh, owh) = match tokio_timeout(timeout.connect, TcpStream::connect(addr)).await {
            Ok(Ok(stream)) => stream.into_split(),
            Ok(Err(e)) => {
                return Err(Report::from(e)
                    .attach_printable("Failed to establish a TCP connection")
                    .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                    .change_context(
                        NetworkError::ConnectionError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ));
            }
            Err(e) => {
                return Err(Report::from(e)
                    .attach_printable("Connection operation timed out")
                    .attach_printable(format!("Attempted to connect to address: {addr:?}"))
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ));
            }
        };

        Ok(AsyncTokioTcpClient {
            read_stream: Arc::new(Mutex::new(orh)),
            read_timeout: timeout.read,
            write_stream: Arc::new(Mutex::new(owh)),
            write_timeout: timeout.write,
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>> {
        let read_half = Arc::clone(&self.read_stream);
        let mut orh = read_half.lock().await;

        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        match tokio_timeout(self.read_timeout, orh.read_to_end(&mut buf)).await {
            Ok(Ok(_)) => Ok(buf),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to read data from its half of the TCP split stream")
                    .change_context(
                        NetworkError::ReadError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Read operation timed out")
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ))
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<()> {
        let write_half = Arc::clone(&self.write_stream);
        let mut owh = write_half.lock().await;

        match tokio_timeout(self.write_timeout, owh.write_all(data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                Err(Report::from(e)
                    .attach_printable("Failed to write data to its half of the TCP split stream")
                    .change_context(
                        NetworkError::WriteError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ))
            }
            Err(e) => {
                Err(Report::from(e)
                    .attach_printable("Write operation timed out")
                    .change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            _interface: NetworkInterface::SealedClientTokio,
                        }
                        .into(),
                    ))
            }
        }
    }
}
