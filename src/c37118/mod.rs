pub mod decoder;
mod deserializer;
pub mod encoder;
pub mod error;
pub mod message;
mod serializer;

pub const MAX_FRAMESIZE: usize = 65535;
