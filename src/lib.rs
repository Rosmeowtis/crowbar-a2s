#[cfg(feature = "sync")]
pub mod client;
#[cfg(feature = "async")]
pub mod client_async;
pub mod constants;
pub mod errors;
pub mod types;

#[cfg(feature = "sync")]
pub use crate::client::A2SClient;
#[cfg(feature = "async")]
pub use crate::client_async::A2SClientAsync;
use crate::errors::Result;
use std::time::Duration;

#[cfg(feature = "sync")]
use std::net::UdpSocket;

pub struct Builder {
    max_size: usize,
    app_id: u16,
    timeout: Duration,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            max_size: 1400,
            app_id: 0,
            timeout: Duration::new(5, 0),
        }
    }

    pub fn max_size(&mut self, size: usize) -> &mut Self {
        self.max_size = size;
        self
    }

    pub fn app_id(&mut self, app_id: u16) -> &mut Self {
        self.app_id = app_id;
        self
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    #[cfg(feature = "sync")]
    pub fn build_sync(&self) -> Result<A2SClient> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let timeout = self.timeout;

        socket.set_read_timeout(Some(timeout))?;
        socket.set_write_timeout(Some(timeout))?;

        Ok(A2SClient {
            socket,
            max_size: self.max_size,
            app_id: self.app_id,
        })
    }

    #[cfg(feature = "async")]
    pub fn build_async(&self) -> Result<A2SClientAsync> {
        Ok(A2SClientAsync {
            timeout: self.timeout,
            max_size: self.max_size,
            app_id: self.app_id,
        })
    }
}
