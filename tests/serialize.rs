use test_log::test;

use serde_synphasor::{error::SerializeError, serializer::ByteContainer, *};

/**
 * Statically allocated ByteContainer for tests
*/
struct StaticContainer {
    bytes: [u8; StaticContainer::MAX_SIZE],
    index: usize,
}

impl StaticContainer {
    const MAX_SIZE: usize = 65535;
    pub fn new() -> StaticContainer {
        StaticContainer {
            bytes: [0; StaticContainer::MAX_SIZE],
            index: 0,
        }
    }
}
impl ByteContainer for StaticContainer {
    fn enque(&mut self, v: u8) -> Result<(), error::SerializeError> {
        if self.index < StaticContainer::MAX_SIZE {
            self.bytes[self.index] = v;
            self.index += 1;
            Ok(())
        } else {
            Err(SerializeError::SpaceExceeded)
        }
    }

    fn get(&self) -> &[u8] {
        &self.bytes[0..self.index]
    }
}

/// Tests Basic Baseframe Serialization.
/// Valid checksum validation is ignored in test
#[test]
fn base_frame_serialization() {
    let message = Message {
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
        data: DataType::Cmd,
    };

    let bytes = StaticContainer::new();

    let serializer = SynSerializer::new(bytes);

    //Ignore checksum, only include first 14 bytes
    assert_eq!(
        [0xAA, 0x41, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5],
        serializer.to_bytes(&message).unwrap().get()[..14]
    );
}
