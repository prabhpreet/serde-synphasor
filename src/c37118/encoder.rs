use super::error::SerializeError;
use super::message::Message;
use super::serializer::{SynSerializer, MAX_FRAMESIZE};
use crate::Container;

pub struct Encoder<C>
where
    C: Container<u8, MAX_FRAMESIZE>,
{
    allocator: EncoderAllocator<C>,
}

impl<C> Encoder<C>
where
    C: Container<u8, MAX_FRAMESIZE>,
{
    pub fn new(allocator: EncoderAllocator<C>) -> Encoder<C> {
        Encoder { allocator }
    }
    pub fn encode(&self, message: Message) -> Result<C, SerializeError> {
        let container = self.allocator.allocate();
        let serializer = SynSerializer::new(container);
        serializer.into_bytes(message)
    }
}

pub struct EncoderAllocator<C>
where
    C: Container<u8, MAX_FRAMESIZE>,
{
    create_serialize_container: fn() -> C,
}

impl<C> EncoderAllocator<C>
where
    C: Container<u8, MAX_FRAMESIZE>,
{
    pub fn new(create_serialize_container: fn() -> C) -> EncoderAllocator<C> {
        EncoderAllocator {
            create_serialize_container,
        }
    }
    fn allocate(&self) -> C {
        (self.create_serialize_container)()
    }
}
