use crate::ParseError;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
#[serde(into = "Frame")]
pub struct Message {
    pub version: FrameVersion,
    pub idcode: u16,
    pub time: Time,
    pub data: DataType,
}

#[derive(PartialEq, Debug, Serialize)]
struct Frame {
    sync: u16,
    framesize: u16,
    idcode: u16,
    soc: u32,
    fracsec: u32,
    data: DataType,
    //Checksum is manually added by Serialize trait method implementation
}

impl From<Message> for Frame {
    fn from(message: Message) -> Self {
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
            DataType::Cmd => 4,
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
        let fracsec = message.time.encode_fracsec_word();

        // Calculate framesize
        let framesize = FRAME_OVERHEAD;

        Frame {
            sync,
            framesize,
            idcode: message.idcode,
            soc: message.time.soc,
            fracsec,
            data: message.data,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Clone)]
pub enum FrameVersion {
    Std2005,
    Std2011,
}

#[derive(PartialEq, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct u24(u32);

impl u24 {
    pub fn new(i: u32) -> Result<u24, ParseError> {
        let base: u32 = 2;
        if i < (base.pow(24)) {
            Ok(u24(i))
        } else {
            //FRACSEC: Fracsec value much greater than 2^24 -1
            Err(ParseError::TypeRangeOverflow)
        }
    }
    pub fn encode(&self) -> u32 {
        self.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Time {
    pub soc: u32,
    pub fracsec: u24,
    pub leap_second_direction: bool,
    pub leap_second_occured: bool,
    pub leap_second_pending: bool,
    pub time_quality: TimeQuality,
}

impl Time {
    fn encode_fracsec_word(&self) -> u32 {
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
        fracsec
    }
}

#[derive(PartialEq, Debug, Serialize, Clone)]
pub enum TimeQuality {
    Fault,    //Fault- clock failure, time not reliable
    UTC10s,   //Time within 10s of UTC
    UTC1s,    //Time within 1s of UTC
    UTC100ms, //Time within 100ms of UTC
    UTC10ms,  //Time within 10ms of UTC
    UTC1ms,   //Time within 1ms of UTC
    UTC100us, //Time within 100us of UTC
    UTC10us,  //Time within 10us of UTC
    UTC1us,   //Time within 1us of UTC
    UTC100ns, //Time within 100ns of UTC
    UTC10ns,  //Time within 10ns of UTC
    UTC1ns,   //Time within 1ns of UTC
    Locked,   //Normal operation, clock locked to UTC traceable source
}

#[derive(PartialEq, Debug, Serialize, Clone)]
pub enum DataType {
    Header,
    Cfg1,
    Cfg2,
    Cfg3,
    Data,
    Cmd,
}
