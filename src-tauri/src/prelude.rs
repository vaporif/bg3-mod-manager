pub use crate::error::Error;
pub use log::*;
pub type Result<T> = core::result::Result<T, Error>;
