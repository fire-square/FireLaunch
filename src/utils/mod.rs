//! Various utilities for the library.
//!
//! This module contains various utilities for the library, such as
//! logging setup, hash calculation, etc.

pub mod crypto;
pub mod ipfs;
pub mod log;

pub use self::log::init_logging;
pub use ipfs::ipfs;
