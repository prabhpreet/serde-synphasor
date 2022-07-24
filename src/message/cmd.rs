use crate::{cmd, error::BaseParseError, ParseError, SerializeError};
use serde::{de::Error, Deserialize, Serialize};

pub(in crate::message) fn deserialize_cmd_type<'de, D>(deserializer: D) -> Result<CmdType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use core::marker::PhantomData;
    struct ValueVisitor<'de> {
        phantom: PhantomData<&'de ()>,
    }
    impl<'de> ValueVisitor<'de> {
        pub fn new() -> ValueVisitor<'de> {
            ValueVisitor {
                phantom: PhantomData,
            }
        }
    }
    impl<'de> serde::de::Visitor<'de> for ValueVisitor<'de> {
        type Value = u16;
        fn expecting(
            &self,
            _: &mut core::fmt::Formatter<'_>,
        ) -> core::result::Result<(), core::fmt::Error> {
            todo!()
        }
        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    struct ByteVisitor<'a, 'de> {
        bytes: &'a mut [u8],
        phantom: PhantomData<&'de ()>,
    }
    impl<'a, 'de> ByteVisitor<'a, 'de> {
        pub fn new(bytes: &'a mut [u8]) -> ByteVisitor<'a, 'de> {
            ByteVisitor {
                bytes,
                phantom: PhantomData,
            }
        }
    }
    impl<'a, 'de> serde::de::Visitor<'de> for ByteVisitor<'a, 'de> {
        type Value = usize;
        fn expecting(
            &self,
            _: &mut core::fmt::Formatter<'_>,
        ) -> core::result::Result<(), core::fmt::Error> {
            todo!()
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.bytes[0..v.len()].copy_from_slice(v);
            if v.len() < 2 {
                Err(E::missing_field("CMD"))
            } else {
                Ok(v.len())
            }
        }
    }

    let mut bytes: [u8; 65520] = [0; 65520];
    let bytes_visitor = ByteVisitor::new(&mut bytes);
    let len = deserializer.deserialize_bytes(bytes_visitor)?;
    let cmd_value: [u8; 2] = bytes[0..2].try_into().unwrap();
    let bytes: [u8; 65518] = bytes[2..].try_into().unwrap();

    let cmd_value = u16::from_be_bytes(cmd_value);

    Ok(match cmd_value {
        1 => CmdType::TurnOffDataFrames,
        2 => CmdType::TurnOnDataFrames,
        3 => CmdType::SendHdrFrame,
        4 => CmdType::SendCfg1Frame,
        5 => CmdType::SendCfg2Frame,
        6 => CmdType::SendCfg3Frame,
        8 => unimplemented!(),
        v @ 256..=4095 => CmdType::UserDesignatedCode(v),
        v => CmdType::ReservedUndesignatedCode(v),
    })
}

pub(in crate::message) fn serialize_cmd_type<'a, S>(
    cmd_type: &CmdType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes: [u8; 65520] = [0; 65520]; //Max allowable size: 2 bytes CMD + 65518 extended frame
    let mut len: usize = 2;
    let cmd: u16 = match cmd_type {
        CmdType::TurnOffDataFrames => 1,
        CmdType::TurnOnDataFrames => 2,
        CmdType::SendHdrFrame => 3,
        CmdType::SendCfg1Frame => 4,
        CmdType::SendCfg2Frame => 5,
        CmdType::SendCfg3Frame => 6,
        CmdType::ExtendedFrame => {
            unimplemented!();
            8
        }
        CmdType::UserDesignatedCode(v) if *v > 256 && *v <= 4095 => *v,
        CmdType::ReservedUndesignatedCode(v)
            if !((*v > 256 && *v <= 4095)
                | (*v == 1)
                | (*v == 2)
                | (*v == 3)
                | (*v == 4)
                | (*v == 5)
                | (*v == 6)
                | (*v == 8)) =>
        {
            *v
        }
        _ => {
            unreachable!()
        }
    };

    bytes[0..2].copy_from_slice(&u16::to_be_bytes(cmd)[..]);
    serializer.serialize_bytes(&bytes[0..len])
}

#[derive(PartialEq, Debug, Clone)]
pub enum CmdType {
    TurnOffDataFrames,
    TurnOnDataFrames,
    SendHdrFrame,
    SendCfg1Frame,
    SendCfg2Frame,
    SendCfg3Frame,
    ExtendedFrame,
    UserDesignatedCode(u16),
    ReservedUndesignatedCode(u16),
}
