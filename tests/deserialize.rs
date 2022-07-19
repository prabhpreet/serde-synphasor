use test_log::test;

use serde_synphasor::*;
#[test]
fn base_frame_deserialization() {
    let bytes: [u8; 16] = [
        0xAA, 0x41, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x23,
        0x7E,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);

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

    assert_eq!(deserialized_message, Ok(message));
}

#[test]
fn base_frame_deserialization_error_checksum() {
    let bytes: [u8; 16] = [
        0xAA, 0x41, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x23,
        0x7F,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(deserialized_message, Err(ParseError::InvalidChecksum));
}

#[test]
fn base_frame_deserialization_error_sync() {
    let bytes: [u8; 16] = [
        0xAB, 0x41, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x58,
        0x1F,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(
            BaseParseError::IncorrectSyncWord
        ))
    );
}

#[test]
fn base_frame_deserialization_error_unknown_frame_type() {
    let bytes: [u8; 16] = [
        0xAA, 0x62, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0xFA,
        0x6F,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(BaseParseError::UnknownFrameType))
    );
}

#[test]
fn base_frame_deserialization_error_frame_version() {
    let bytes: [u8; 16] = [
        0xAA, 0x43, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0xA9,
        0xB8,
    ];
    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(
            BaseParseError::UnknownFrameVersionNumber
        ))
    );
}

#[test]
fn base_frame_deserialization_error_unknown_time_quality() {
    let bytes: [u8; 16] = [
        0xAA, 0x41, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x0C, 0x34, 0x2e, 0xd5, 0x6C,
        0x4C,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(
            BaseParseError::UnknownTimeQuality
        ))
    );
}
