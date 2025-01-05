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
