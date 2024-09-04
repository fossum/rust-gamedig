mod check;
pub(crate) mod io;
pub(crate) mod net;

#[allow(unused_imports)]
#[cfg(feature = "_BUFFER")]
pub(crate) use io::buf::Buffer;

#[allow(unused_imports)]
#[cfg(feature = "_TCP")]
pub(crate) use net::tcp::TcpClient;