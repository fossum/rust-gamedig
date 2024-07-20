use std::{net::SocketAddr, time::Duration};

use async_std::{
    future::timeout as async_timeout,
    io::{ReadExt, WriteExt},
    net::TcpStream,
};

use crate::{
    error::{NetworkError, Report, Result, _metadata::NetworkProtocol},
    settings::Timeout,
};

#[derive(Debug)]
pub(crate) struct AsyncStdTcpClient {
    addr: SocketAddr,
    stream: TcpStream,
    read_timeout: Duration,
    write_timeout: Duration,
}

#[maybe_async::async_impl]
impl super::Tcp for AsyncStdTcpClient {
    async fn new(addr: &SocketAddr, timeout: &Timeout) -> Result<Self> {
        Ok(Self {
            addr: *addr,
            stream: match async_timeout(timeout.connect, TcpStream::connect(addr)).await {
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    return Err(Report::from(e).change_context(
                        NetworkError::ConnectionError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: *addr,
                        }
                        .into(),
                    ));
                }
                Err(e) => {
                    return Err(Report::from(e).change_context(
                        NetworkError::TimeoutElapsedError {
                            _protocol: NetworkProtocol::Tcp,
                            addr: *addr,
                        }
                        .into(),
                    ));
                }
            },
            read_timeout: timeout.read,
            write_timeout: timeout.write,
        })
    }

    async fn read(&mut self, size: Option<usize>) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(size.unwrap_or(Self::DEFAULT_PACKET_SIZE as usize));

        match async_timeout(self.read_timeout, self.stream.read_to_end(&mut buf)).await {
            Ok(Ok(_)) => Ok(buf),
            Ok(Err(e)) => {
                Err(Report::from(e).change_context(
                    NetworkError::ReadError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::TimeoutElapsedError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }

    async fn write(&mut self, data: &[u8]) -> Result<()> {
        match async_timeout(self.write_timeout, self.stream.write_all(data)).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                Err(Report::from(e).change_context(
                    NetworkError::WriteError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
            Err(e) => {
                Err(Report::from(e).change_context(
                    NetworkError::TimeoutElapsedError {
                        _protocol: NetworkProtocol::Tcp,
                        addr: self.addr,
                    }
                    .into(),
                ))
            }
        }
    }
}
