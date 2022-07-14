use crate::{error::*, Frame, Message};
use log::trace;
use serde::{
    de::{IntoDeserializer, SeqAccess},
    Deserialize, Deserializer,
};

pub fn from_bytes(bytes: &[u8]) -> Result<Message, ParseError> {
    let mut deserializer = SynDeserializer::new(&bytes[..bytes.len() - 2]);
    let frame = Frame::deserialize(&mut deserializer)?;
    let checksum = bytes[bytes.len() - 2..]
        .try_into()
        .map_err(|_| ParseError::IllegalAccess)?;
    let checksum = u16::from_be_bytes(checksum);
    if checksum == deserializer.get_checksum() {
        let message = frame.try_into()?;
        Ok(message)
    } else {
        trace!("{:x}", deserializer.get_checksum());
        Err(ParseError::InvalidChecksum)
    }
}
pub struct SynDeserializer<'de> {
    bytes: &'de [u8],
    index: usize,
    sync: Option<u16>,
    checksum: u16,
}

impl<'de> SynDeserializer<'de> {
    pub fn new(bytes: &'de [u8]) -> SynDeserializer {
        SynDeserializer {
            bytes,
            index: 0,
            sync: None,
            checksum: 0xFF_FF,
        }
    }

    fn enque_checksum(&mut self, bytes: &[u8]) {
        for &v in bytes {
            let mut chk = self.checksum;
            let temp = (chk >> 8) ^ (v as u16);
            chk <<= 8;
            let mut quick = temp ^ (temp >> 4);
            chk ^= quick;
            quick <<= 5;
            chk ^= quick;
            quick <<= 7;
            chk ^= quick;
            self.checksum = chk;
        }
    }

    pub(crate) fn get_checksum(&self) -> u16 {
        self.checksum
    }
}

struct SynDeserializerSeqAccess<'a, 'de: 'a> {
    deserializer: &'a mut SynDeserializer<'de>,
}

impl<'a, 'de> SynDeserializerSeqAccess<'a, 'de> {
    fn new(deserializer: &'a mut SynDeserializer<'de>) -> SynDeserializerSeqAccess<'a, 'de> {
        SynDeserializerSeqAccess::<'a, 'de> { deserializer }
    }
}

impl<'de, 'a> SeqAccess<'de> for SynDeserializerSeqAccess<'a, 'de> {
    type Error = ParseError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.deserializer)?;
        Ok(Some(value))
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut SynDeserializer<'de> {
    type Error = ParseError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let index = self.index;
        let bytes: [u8; 2] = self.bytes[index..index + 2]
            .try_into()
            .map_err(|_| ParseError::IllegalAccess)?;
        self.enque_checksum(&bytes);
        let value = u16::from_be_bytes(bytes);
        self.index += 2;
        if self.sync.is_none() {
            self.sync = Some(value);
        }
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let index = self.index;
        let bytes: [u8; 4] = self.bytes[index..index + 4]
            .try_into()
            .map_err(|_| ParseError::IllegalAccess)?;
        self.enque_checksum(&bytes);
        let value = u32::from_be_bytes(bytes);
        self.index += 4;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SynDeserializerSeqAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        trace!("{}", name);
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        trace!("{:?}\n{:?}", name, variants);
        if name == "DataType" {
            if let Some(sync) = self.sync {
                let frame_type = match (sync & 0x0070u16) >> 4 {
                    0 => "Data",
                    1 => "Header",
                    2 => "Cfg1",
                    3 => "Cfg2",
                    4 => "Cmd",
                    5 => "Cfg3",
                    _ => {
                        return Err(ParseError::BaseParseError(BaseParseError::UnknownFrameType));
                    }
                };
                visitor.visit_enum(frame_type.into_deserializer())
            } else {
                return Err(ParseError::BaseParseError(BaseParseError::UnknownFrameType));
            }
        } else {
            todo!()
        }
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
