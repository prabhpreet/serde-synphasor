use test_log::test;

use serde_synphasor::{serializer::ByteContainer, *};

/**
 * Dynamically allocated ByteContainer for tests
*/
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
impl ByteContainer for VecContainer {
    fn enque(&mut self, v: u8) -> Result<(), error::SerializeError> {
        self.bytes.push(v);
        self.index += 1;
        Ok(())
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
        data: DataType::Cmd(CmdType::TurnOffDataFrames),
    };

    let bytes = VecContainer::new();

    let serializer = SynSerializer::new(bytes);

    //Ignore checksum, only include first 14 bytes
    assert_eq!(
        [
            0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5,
            0x00, 0x01, 0x56, 0x0b
        ][..16],
        serializer.to_bytes(&message).unwrap().get()[..16]
    );
}
