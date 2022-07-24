#![doc(include = "../README.md")]
#![no_std]
pub mod config;
pub mod deserializer;
pub mod error;
pub mod message;
pub mod serializer;
pub use crate::config::Config;
pub use crate::error::*;
pub use crate::message::*;
pub use crate::serializer::SynSerializer;

/// Generic Container for buffered storage
pub trait Container<T>
where
    T: Sized,
{
    fn enque(&mut self, v: T) -> Result<(), SerializeError>;
    fn get(&self) -> &[T];
}
