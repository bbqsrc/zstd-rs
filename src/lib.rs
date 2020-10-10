//! Rust binding to the [zstd library][zstd].
//!
//! This crate provides:
//!
//! * An [encoder](stream/write/struct.Encoder.html) to compress data using zstd
//!   and send the output to another write.
//! * A [decoder](stream/read/struct.Decoder.html) to read input data from a `Read`
//!   and decompress it.
//! * Convenient functions for common tasks.
//!
//! # Example
//!
//! ```no_run
//! extern crate zstd;
//!
//! fn main() {
//! 	// Uncompress input and print the result.
//! 	zstd::stream::copy_decode(std::io::stdin(), std::io::stdout()).unwrap();
//! }
//! ```
//!
//! [zstd]: https://github.com/facebook/zstd
#![deny(missing_docs)]
#![feature(min_const_generics)]

pub mod block;
pub mod dict;
pub mod stream;

use bare_io as io;

/// Default compression level.
pub use zstd_safe::CLEVEL_DEFAULT as DEFAULT_COMPRESSION_LEVEL;

#[doc(no_inline)]
#[cfg(feature = "std")]
pub use crate::stream::{decode_all, encode_all};
pub use crate::stream::{Decoder, Encoder};

/// Returns the error message as io::Error based on error_code.
fn map_error_code(code: usize) -> io::Error {
    let msg = zstd_safe::get_error_name(code);
    #[cfg(feature = "std")]
    { io::Error::new(io::ErrorKind::Other, msg.to_string()) }
    #[cfg(not(feature = "std"))]
    { io::Error::new(io::ErrorKind::Other, msg) }
}

// Some helper functions to write full-cycle tests.

#[cfg(test)]
fn test_cycle<F, G>(data: &[u8], f: F, g: G)
where
    F: Fn(&[u8]) -> Vec<u8>,
    G: Fn(&[u8]) -> Vec<u8>,
{
    let mid = f(data);
    let end = g(&mid);
    assert_eq!(data, &end[..]);
}

#[cfg(test)]
fn test_cycle_unwrap<F, G>(data: &[u8], f: F, g: G)
where
    F: Fn(&[u8]) -> io::Result<Vec<u8>>,
    G: Fn(&[u8]) -> io::Result<Vec<u8>>,
{
    test_cycle(data, |data| f(data).unwrap(), |data| g(data).unwrap())
}
