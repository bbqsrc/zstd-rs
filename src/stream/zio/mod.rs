//! Wrappers around raw operations implementing `bare_io::{Read, Write}`.

mod reader;
mod writer;

pub use self::reader::Reader;
pub use self::writer::Writer;
