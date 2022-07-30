use serde_synphasor::c37118::encoder::{Encoder, EncoderAllocator};
use test_log::test;

use serde::Serialize;
use serde_synphasor::c37118::{message::*, MAX_FRAMESIZE};
use serde_synphasor::{Container, *};

/**
 * Dynamically allocated ByteContainer for tests
*/

#[derive(Debug)]
struct VecContainer {
    bytes: std::vec::Vec<u8>,
    index: usize,
}

impl VecContainer {
    pub fn new() -> VecContainer {
        VecContainer {
            bytes: vec![],
            index: 0,
        }
    }
}
impl Container<u8, MAX_FRAMESIZE> for VecContainer {
    fn enque(&mut self, v: u8) -> Result<(), ContainerError> {
        self.bytes.push(v);
        self.index += 1;
        Ok(())
    }

    fn get(&self) -> &[u8] {
        &self.bytes[0..self.index]
    }
}

#[derive(Debug)]
struct CmdStackContainer {
    bytes: [u8; MAX_EXTENDED_FRAME_SIZE],
    len: usize,
}

impl Serialize for CmdStackContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.serialize_impl(serializer)
    }
}

impl Container<u8, MAX_EXTENDED_FRAME_SIZE> for CmdStackContainer {
    fn enque(&mut self, v: u8) -> Result<(), ContainerError> {
        self.bytes[self.len] = v;
        self.len += 1;
        Ok(())
    }

    fn get(&self) -> &[u8] {
        &self.bytes[0..self.len]
    }
}
impl CmdStore for CmdStackContainer {}

/// Tests Basic Baseframe Serialization.
/// Valid checksum validation is ignored in test
#[test]
fn base_frame_serialization() {
    let message = Message::<CmdStackContainer> {
        version: FrameVersion::Std2005,
        idcode: 60,
        time: Time {
            soc: 1_218_023_578,
            fracsec: u24::new(3419861).unwrap(),
            leap_second_direction: false,
            leap_second_occured: false,
            leap_second_pending: false,
            time_quality: TimeQuality::Locked,
        },
        data: DataType::Cmd(CmdType::TurnOffDataFrames),
    };

    fn vec_allocator() -> VecContainer {
        VecContainer::new()
    }
    let vec_allocator = EncoderAllocator::new(vec_allocator);

    let encoder = Encoder::new(vec_allocator);

    //Ignore checksum, only include first 16 bytes
    assert_eq!(
        [
            0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5,
            0x00, 0x01, 0x56, 0x0b
        ][..16],
        encoder.encode(message).unwrap().get()[..16]
    );
}
