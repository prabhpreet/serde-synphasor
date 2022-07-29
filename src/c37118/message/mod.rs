pub mod cmd;
use crate::c37118::deserializer::SynDeserializer;
use crate::c37118::error::{BaseParseError, ParseError};
use crate::{u24, Time, TimeError, TimeQuality};
pub use cmd::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct Message<'c> {
    pub version: FrameVersion,
    pub idcode: u16,
    pub time: Time,
    pub data: DataType<'c>,
}

#[derive(Debug, Serialize, PartialEq)]
pub(in crate) struct Frame<'c> {
    sync: u16,
    pub(crate) framesize: u16,
    idcode: u16,
    soc: u32,
    fracsec: u32,
    data: DataType<'c>,
}

impl<'c> From<Message<'c>> for Frame<'c> {
    fn from(message: Message<'c>) -> Self {
        const FRAME_OVERHEAD: u16 = 2 + //SYNC
            2 + //FRAMESIZE
            2 + //IDCODE 
            4 + //SOC
            4 + //FRACSEC
            2; //CHK

        // Encode Sync bit

        // Sync: Frame synchronization word.
        // Leading byte: AA hex
        let mut sync: u16 = 0xAA_FFu16;

        //     Second byte: Frame type and version, divided as follows:
        //     Bit 8: Reserved for future definition, must be 0 for this standard version.
        sync &= 0xFF7F;
        //     Bits 7–4:
        //         000: Data Frame
        //         001: Header Frame
        //         010: Configuration Frame 1
        //         011: Configuration Frame 2
        //         100: Configuration Frame 3
        //         101: Command Frame (received message)

        let data_type: u8 = (match message.data {
            DataType::Data => 0,
            DataType::Header => 1,
            DataType::Cfg1 => 2,
            DataType::Cfg2 => 3,
            DataType::Cfg3 => 5,
            DataType::Cmd(_) => 4,
        } << 4)
            | 0x0Fu8;
        sync &= data_type as u16 | 0xFF8F;

        //     Bits 3–0: Version number, in binary (1–15)
        //         Version 2 (0001) for messages defined in IEEE Std C37.118-2005 [B6].
        //         Version 3 (0010) for messages added in this revision,IEEE Std C37.118.2-2011.
        let version: u8 = match message.version {
            FrameVersion::Std2005 => 1,
            FrameVersion::Std2011 => 2,
        } | 0xF0;
        sync &= version as u16 | 0xFFF0;
        let (soc, fracsec) = message.time.encode();

        // TODO: Calculate framesize accurately
        let framesize = FRAME_OVERHEAD + message.data.get_framesize();

        Frame {
            sync,
            framesize,
            idcode: message.idcode,
            soc,
            fracsec,
            data: message.data,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub(in crate) struct FrameDataU8<'a> {
    sync: u16,
    pub(crate) framesize: u16,
    idcode: u16,
    soc: u32,
    fracsec: u32,
    data: &'a [u8],
}

impl<'a, 'c> TryFrom<FrameDataU8<'a>> for Frame<'c> {
    type Error = ParseError;

    fn try_from(value: FrameDataU8) -> Result<Self, Self::Error> {
        let mut deserializer = SynDeserializer::new(value.data);
        let data = match (value.sync & 0x0070u16) >> 4 {
            0 => DataType::Data,
            1 => DataType::Header,
            2 => DataType::Cfg1,
            3 => DataType::Cfg2,
            4 => DataType::Cmd(deserialize_cmd_type(&mut deserializer)?),
            5 => DataType::Cfg3,
            _ => {
                return Err(ParseError::BaseFrame(BaseParseError::UnknownFrameType));
            }
        };

        Ok(Frame {
            sync: value.sync,
            framesize: value.framesize,
            idcode: value.idcode,
            soc: value.soc,
            fracsec: value.fracsec,
            data,
        })
    }
}

impl<'c> TryFrom<Frame<'c>> for Message<'c> {
    type Error = ParseError;

    fn try_from(value: Frame<'c>) -> Result<Self, Self::Error> {
        // Check Sync: Frame synchronization word.
        if (value.sync & 0xFF00) != 0xAA00 {
            return Err(ParseError::BaseFrame(BaseParseError::IncorrectSyncWord));
        }
        //     Second byte: Frame type and version, divided as follows:
        //     Bit 8: Reserved for future definition, must be 0 for this standard version.
        if (value.sync & 0x0080) != 0x0000 {
            return Err(ParseError::BaseFrame(
                BaseParseError::IncorrectReservedSyncBit,
            ));
        }
        //     Bits 4–0: Version number, in binary (1–15)
        let version = match value.sync & 0x000F {
            1 => FrameVersion::Std2005,
            //         Version 2 (0001) for messages defined in IEEE Std C37.118-2005 [B6].
            2 => FrameVersion::Std2011,
            //         Version 3 (0010) for messages added in this revision,IEEE Std C37.118.2-2011.
            _ => {
                return Err(ParseError::BaseFrame(
                    BaseParseError::UnknownFrameVersionNumber,
                ))
            }
        };

        let time = Time::decode(value.soc, value.fracsec)?;

        Ok(Message {
            version,
            idcode: value.idcode,
            time,
            data: value.data,
        })
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub enum FrameVersion {
    Std2005,
    Std2011,
}

impl Time {
    fn encode(&self) -> (u32, u32) {
        // Encode Fracsec
        // Fraction of second and Time Quality, time of measurement for data frames or time
        // of frame transmission for non-data frames.
        // Bits 31–24: Message Time Quality as defined in 6.2.2.
        // Bits 23–00: FRACSEC, 24-bit integer number. When divided by TIME_BASE
        // yields the actual fractional second. FRACSEC used in all messages to and from a
        // given PMU shall use the same TIME_BASE that is provided in the configuration
        // message from that PMU.

        //Bit 31 (Bit 7 as per Table 3 in standard) is reserved so set to zero
        let mut fracsec: u32 = 0u32;
        //Bit 30 (Bit 6 as per Table 3 in standard) of Time Quality Flag
        if self.leap_second_direction {
            fracsec |= 1 << 30;
        }
        if self.leap_second_occured {
            fracsec |= 1 << 29;
        }
        if self.leap_second_pending {
            fracsec |= 1 << 28;
        }

        //Bit 27 to 24 (Bit 3-0 as per Table 3 in standard) of Time Quality Flag
        fracsec |= (match self.time_quality {
            TimeQuality::Fault => 0x0F,
            TimeQuality::UTC10s => 0x0B,
            TimeQuality::UTC1s => 0x0A,
            TimeQuality::UTC100ms => 0x09,
            TimeQuality::UTC10ms => 0x08,
            TimeQuality::UTC1ms => 0x07,
            TimeQuality::UTC100us => 0x06,
            TimeQuality::UTC10us => 0x05,
            TimeQuality::UTC1us => 0x04,
            TimeQuality::UTC100ns => 0x03,
            TimeQuality::UTC10ns => 0x02,
            TimeQuality::UTC1ns => 0x01,
            TimeQuality::Locked => 0x00,
        } << 24);

        //Fracsec: Bits 8 to 32 (Bits 23 to 0 as per Table 3 of standard)
        fracsec |= self.fracsec.encode();
        (self.soc, fracsec)
    }
    fn decode(soc: u32, fracsec: u32) -> Result<Time, ParseError> {
        if (fracsec & 0x80000000) > 0 {
            return Err(ParseError::BaseFrame(
                BaseParseError::IncorrectReservedFracsecBit,
            ));
        }
        let leap_second_direction = (fracsec & 0x40000000) > 0;

        let leap_second_occured = (fracsec & 0x20000000) > 0;
        let leap_second_pending = (fracsec & 0x10000000) > 0;

        let time_quality = match (fracsec & 0x0F000000) >> 24 {
            0x0F => TimeQuality::Fault,
            0x0B => TimeQuality::UTC10s,
            0x0A => TimeQuality::UTC1s,
            0x09 => TimeQuality::UTC100ms,
            0x08 => TimeQuality::UTC10ms,
            0x07 => TimeQuality::UTC1ms,
            0x06 => TimeQuality::UTC100us,
            0x05 => TimeQuality::UTC10us,
            0x04 => TimeQuality::UTC1us,
            0x03 => TimeQuality::UTC100ns,
            0x02 => TimeQuality::UTC10ns,
            0x01 => TimeQuality::UTC1ns,
            0x00 => TimeQuality::Locked,
            _ => return Err(ParseError::BaseFrame(BaseParseError::UnknownTimeQuality)),
        };

        let fracsec = u24::new(fracsec & (0x00FFFFFF))
            .map_err(|e| ParseError::BaseFrame(BaseParseError::Fracsec(e)))?;

        Ok(Time {
            soc,
            fracsec,
            leap_second_direction,
            leap_second_occured,
            leap_second_pending,
            time_quality,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DataType<'a> {
    Header,
    Cfg1,
    Cfg2,
    Cfg3,
    Data,
    #[serde(
        serialize_with = "cmd::serialize_cmd_type",
        deserialize_with = "cmd::deserialize_cmd_type"
    )]
    Cmd(CmdType<'a>),
}

impl<'a> DataType<'a> {
    fn get_framesize(&self) -> u16 {
        match self {
            DataType::Header => todo!(),
            DataType::Cfg1 => todo!(),
            DataType::Cfg2 => todo!(),
            DataType::Cfg3 => todo!(),
            DataType::Data => todo!(),
            DataType::Cmd(cmd) => match cmd {
                CmdType::ExtendedFrame(c) => todo!(),
                _ => 2,
            },
        }
    }
}

#[cfg(test)]
mod serialize_test {

    use super::*;
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn serialize_sync_idcode_soc_fracsec_encoding() {
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
        let frame: Frame = message.into();

        assert_ser_tokens(
            &frame,
            &[
                Token::Struct {
                    name: "Frame",
                    len: 6,
                },
                Token::Str("sync"),
                Token::U16(0xAA41),
                Token::Str("framesize"),
                Token::U16(0x0012),
                Token::Str("idcode"),
                Token::U16(0x003C),
                Token::Str("soc"),
                Token::U32(0x4899909A),
                Token::Str("fracsec"),
                Token::U32(0x00342ED5),
                Token::Str("data"),
                Token::Enum { name: "DataType" },
                Token::Str("Cmd"),
                Token::Bytes(&[0u8, 1u8]),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn serialize_time_quality_check() {
        let message = Message {
            version: FrameVersion::Std2011,
            idcode: 0,
            time: Time {
                soc: 0,
                fracsec: u24::new(0).unwrap(),
                leap_second_direction: true,
                leap_second_occured: true,
                leap_second_pending: true,
                time_quality: TimeQuality::Fault,
            },
            data: DataType::Cmd(CmdType::TurnOffDataFrames),
        };

        let frame: Frame = message.into();
        assert_ser_tokens(
            &frame,
            &[
                Token::Struct {
                    name: "Frame",
                    len: 6,
                },
                Token::Str("sync"),
                Token::U16(0xAA42),
                Token::Str("framesize"),
                Token::U16(0x0012),
                Token::Str("idcode"),
                Token::U16(0x0000),
                Token::Str("soc"),
                Token::U32(0x00000000),
                Token::Str("fracsec"),
                Token::U32(0x7F000000),
                Token::Str("data"),
                Token::Enum { name: "DataType" },
                Token::Str("Cmd"),
                Token::Bytes(&[0u8, 1u8]),
                Token::StructEnd,
            ],
        );
    }
}

#[cfg(test)]
mod deserialize_test {

    use super::*;
    use serde_test::{assert_de_tokens, Token};

    #[test]
    fn decode_error_fracsec_reserved_bit_set() {
        let t = Time::decode(0, 0x80000000);
        assert_eq!(
            t,
            Err(ParseError::BaseFrame(
                BaseParseError::IncorrectReservedFracsecBit
            ))
        );
    }

    #[test]
    fn decode_error_fracsec_unknown_time_quality() {
        let t = Time::decode(0, 0x0C000000);
        assert_eq!(
            t,
            Err(ParseError::BaseFrame(BaseParseError::UnknownTimeQuality))
        );
    }

    #[test]
    fn deserialize_sample_message() {
        let message = Message {
            version: FrameVersion::Std2011,
            idcode: 0,
            time: Time {
                soc: 0,
                fracsec: u24::new(0).unwrap(),
                leap_second_direction: true,
                leap_second_occured: true,
                leap_second_pending: true,
                time_quality: TimeQuality::Fault,
            },
            data: DataType::Cmd(CmdType::TurnOffDataFrames),
        };
        let frame: Frame = message.into();

        let frame_u8 = FrameDataU8 {
            sync: frame.sync,
            idcode: frame.idcode,
            soc: frame.soc,
            fracsec: frame.fracsec,
            framesize: frame.framesize,
            data: &[0x01],
        };

        assert_de_tokens(
            &frame_u8,
            &[
                Token::Struct {
                    name: "FrameDataU8",
                    len: 6,
                },
                Token::Str("sync"),
                Token::U16(0xAA42),
                Token::Str("framesize"),
                Token::U16(0x0012),
                Token::Str("idcode"),
                Token::U16(0x0000),
                Token::Str("soc"),
                Token::U32(0x00000000),
                Token::Str("fracsec"),
                Token::U32(0x7F000000),
                Token::Str("data"),
                Token::BorrowedBytes(&[0x01]),
                Token::StructEnd,
            ],
        );
    }
}
