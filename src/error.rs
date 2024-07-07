//! Expected errors
//!
#![deny(missing_docs)]

#[derive(Debug, thiserror::Error)]
/// Possible errors
enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Generic error
    #[cfg_attr(error("Undefined error: {0}"))]
    Undefined(String),
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
