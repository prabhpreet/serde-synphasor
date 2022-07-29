use super::deserializer::SynDeserializer;
use super::error::*;
use super::message::{Frame, FrameDataU8, Message, MAX_EXTENDED_FRAME_SIZE};
use super::MAX_FRAMESIZE;
use crate::{create_phantom_container, Container, PhantomContainer};
use log::trace;
use serde::Deserialize;
pub struct Decoder<CmdExtendedFrame>
where
    CmdExtendedFrame: Container<u8, MAX_EXTENDED_FRAME_SIZE>,
{
    allocator: DecoderAllocator<CmdExtendedFrame>,
}

impl<CmdContainer> Decoder<CmdContainer>
where
    CmdContainer: Container<u8, MAX_EXTENDED_FRAME_SIZE>,
{
    pub fn new(allocator: DecoderAllocator<CmdContainer>) -> Decoder<CmdContainer> {
        Decoder { allocator }
    }
    pub fn decode<C>(&self, deserialize_container: &C) -> Result<Message, ParseError>
    where
        C: Container<u8, MAX_FRAMESIZE>,
    {
        //let container = self.allocator.allocate();
        //let serializer = SynDeserializer::new(container);
        //serializer.into_bytes(message)

        let bytes = deserialize_container.get();
        let bytes_len: u16 = bytes
            .len()
            .try_into()
            .map_err(|_| ParseError::BytesExceedFrameSize)?;
        let mut deserializer = SynDeserializer::new(&bytes[..bytes.len() - 2]);
        let frame = FrameDataU8::deserialize(&mut deserializer)?;
        if frame.framesize != bytes_len {
            return Err(ParseError::InvalidFrameSize);
        }
        let checksum = bytes[bytes.len() - 2..]
            .try_into()
            .map_err(|_| ParseError::IllegalAccess)?;
        let checksum = u16::from_be_bytes(checksum);
        if checksum == deserializer.get_checksum() {
            let frame: Frame = frame.try_into()?;
            let message = frame.try_into()?;
            Ok(message)
        } else {
            trace!("{:x}", deserializer.get_checksum());
            Err(ParseError::InvalidChecksum)
        }
    }
}

pub struct DecoderAllocator<CmdContainer>
where
    CmdContainer: Container<u8, MAX_EXTENDED_FRAME_SIZE>,
{
    create_cmd_framesize_container: fn() -> CmdContainer,
}

impl<CmdContainer> DecoderAllocator<CmdContainer>
where
    CmdContainer: Container<u8, MAX_EXTENDED_FRAME_SIZE>,
{
    pub fn new(allocator: fn() -> CmdContainer) -> DecoderAllocator<CmdContainer> {
        DecoderAllocator {
            create_cmd_framesize_container: allocator,
        }
    }
}
