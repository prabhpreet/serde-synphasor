pub mod config;
pub mod error;
pub mod message;
pub mod serializer;
pub use crate::config::Config;
pub use crate::error::ParseError;
pub use crate::message::*;
pub use crate::serializer::SynSerializer;

pub fn from_bytes(_bytes: &[u8], _config: Option<Config>) -> Result<Message, ParseError> {
    unimplemented!()
}
