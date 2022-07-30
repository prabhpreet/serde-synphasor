use serde_synphasor::c37118::decoder::{Decoder, DecoderAllocator};
use test_log::test;

use serde_synphasor::c37118::{message::*, MAX_FRAMESIZE};
use serde_synphasor::{Container, *};

use serde_synphasor::c37118::{error::*, message::*};

#[derive(Debug)]
struct ReadableStackContainer<const SIZE: usize> {
    bytes: [u8; SIZE],
}
impl<const SIZE: usize> ReadableStackContainer<SIZE> {
    pub fn new(bytes: [u8; SIZE]) -> ReadableStackContainer<SIZE> {
        ReadableStackContainer { bytes }
    }
}
impl<const SIZE: usize> Container<u8, MAX_FRAMESIZE> for ReadableStackContainer<SIZE> {
    fn enque(&mut self, _v: u8) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get(&self) -> &[u8] {
        &self.bytes
    }
}

#[test]
fn base_frame_deserialization_ok() {
    let bytes: [u8; 18] = [
        0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x56, 0x0b,
    ];

    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);
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

    assert_eq!(deserialized_message, Ok(message));
}

#[test]
fn base_frame_deserialization_error_checksum() {
    let bytes: [u8; 18] = [
        0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x56, 0x0c,
    ];

    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);
    assert_eq!(deserialized_message, Err(ParseError::InvalidChecksum));
}

#[test]
fn base_frame_deserialization_error_sync() {
    let bytes: [u8; 18] = [
        0xAB, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0xFE, 0x2F,
    ];
    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseFrame(BaseParseError::IncorrectSyncWord))
    );
}

#[test]
fn base_frame_deserialization_error_unknown_frame_type() {
    let bytes: [u8; 16] = [
        0xAA, 0x62, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0xC1,
        0xB3,
    ];

    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);
    let equal =
        deserialized_message == Err(ParseError::BaseFrame(BaseParseError::UnknownFrameType));
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseFrame(BaseParseError::UnknownFrameType))
    );
}

#[test]
fn base_frame_deserialization_error_frame_version() {
    let bytes: [u8; 18] = [
        0xaa, 0x43, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x1B, 0xD2,
    ];

    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseFrame(
            BaseParseError::UnknownFrameVersionNumber
        ))
    );
}

#[test]
fn base_frame_deserialization_error_unknown_time_quality() {
    let bytes: [u8; 18] = [
        0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x0c, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x5d, 0xe8,
    ];

    let container = ReadableStackContainer::new(bytes);
    let decoder = Decoder::new(DecoderAllocator::new(create_phantom_container));

    let deserialized_message = decoder.decode(&container);

    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseFrame(BaseParseError::UnknownTimeQuality))
    );
}
