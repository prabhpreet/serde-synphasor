use test_log::test;

use serde_synphasor::*;
#[test]
fn base_frame_deserialization_ok() {
    let bytes: [u8; 18] = [
        0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x56, 0x0b,
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

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(deserialized_message, Err(ParseError::InvalidChecksum));
}

#[test]
fn base_frame_deserialization_error_sync() {
    let bytes: [u8; 18] = [
        0xAB, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0xFE, 0x2F,
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
        0xAA, 0x62, 0x00, 0x10, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0xC1,
        0xB3,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(BaseParseError::UnknownFrameType))
    );
}

#[test]
fn base_frame_deserialization_error_frame_version() {
    let bytes: [u8; 18] = [
        0xaa, 0x43, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x1B, 0xD2,
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
    let bytes: [u8; 18] = [
        0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x0c, 0x34, 0x2e, 0xd5, 0x00,
        0x01, 0x5d, 0xe8,
    ];

    let deserialized_message = deserializer::from_bytes(&bytes);
    assert_eq!(
        deserialized_message,
        Err(ParseError::BaseParseError(
            BaseParseError::UnknownTimeQuality
        ))
    );
}
