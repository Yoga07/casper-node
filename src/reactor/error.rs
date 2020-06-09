use std::result;

use thiserror::Error;

use crate::components::small_network;

pub(crate) type Result<T> = result::Result<T, Error>;

/// Error type returned by a reactor.
#[derive(Debug, Error)]
pub enum Error {
    /// `SmallNetwork` component error.
    #[error("small network error: {0}")]
    SmallNetwork(#[from] small_network::Error),
}
