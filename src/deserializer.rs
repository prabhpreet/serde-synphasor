use crate::{error::*, Frame, Message};
use log::trace;
use serde::{
    de::{EnumAccess, IntoDeserializer, SeqAccess, VariantAccess, Visitor},
    Deserialize, Deserializer,
};

pub fn from_bytes(bytes: &[u8]) -> Result<Message, ParseError> {
    let bytes_len: u16 = bytes
        .len()
        .try_into()
        .map_err(|_| ParseError::BytesExceedFrameSize)?;
    let mut deserializer = SynDeserializer::new(&bytes[..bytes.len() - 2]);
    let frame = Frame::deserialize(&mut deserializer)?;
    if (frame.framesize != bytes_len) {
        return Err(ParseError::InvalidFrameSize);
    }
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

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
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

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let index = self.index;
        let size = self.bytes.len() - index;
        let bytes = &self.bytes[index..];
        self.enque_checksum(&bytes);
        self.index += size;
        visitor.visit_bytes(&bytes)
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
                trace!("Deserialize_Enum: Before visit enum");
                let frame_type = match (sync & 0x0070u16) >> 4 {
                    0 => visitor.visit_enum(SynDeserializerEA::<false>::new(self)), //Data
                    1 => visitor.visit_enum(SynDeserializerEA::<false>::new(self)), //Header
                    2 => visitor.visit_enum(SynDeserializerEA::<false>::new(self)), //Cfg1
                    3 => visitor.visit_enum(SynDeserializerEA::<false>::new(self)), //Cfg2
                    4 => visitor.visit_enum(SynDeserializerEA::<true>::new(self)),  //Cmd
                    5 => visitor.visit_enum(SynDeserializerEA::<false>::new(self)), //Cfg3
                    _ => {
                        return Err(ParseError::BaseParseError(BaseParseError::UnknownFrameType));
                    }
                };

                trace!("After visit enum");
                frame_type
            } else {
                Err(ParseError::BaseParseError(
                    BaseParseError::IncorrectSyncWord,
                ))
            }
        } else {
            todo!()
        }
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        //_visitor.visit_newtype_struct(self)
        print_type_of(&_visitor);
        if let Some(sync) = self.sync {
            trace!("VisitIdentifier");
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
            _visitor.visit_str(frame_type)
        } else {
            Err(ParseError::BaseParseError(
                BaseParseError::IncorrectSyncWord,
            ))
        }

        /*
        todo!()

        */
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}

struct SynDeserializerEA<'a, 'de: 'a, const NEWTYPE: bool> {
    deserializer: &'a mut SynDeserializer<'de>,
}

impl<'a, 'de, const NEWTYPE: bool> SynDeserializerEA<'a, 'de, NEWTYPE> {
    fn new(deserializer: &'a mut SynDeserializer<'de>) -> Self {
        trace!("EA Initialized");
        SynDeserializerEA { deserializer }
    }
}

impl<'a, 'de, const NEWTYPE: bool> EnumAccess<'de> for SynDeserializerEA<'a, 'de, NEWTYPE> {
    type Error = ParseError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        trace!("Variant seed");
        print_type_of(&seed);
        let val = seed.deserialize(&mut *self.deserializer)?;
        Ok((val, self))
    }
}

impl<'a, 'de, const NEWTYPE: bool> VariantAccess<'de> for SynDeserializerEA<'a, 'de, NEWTYPE> {
    type Error = ParseError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        trace!("Unit Variant");
        if NEWTYPE {
            Err(ParseError::InvalidEnumVariant)
        } else {
            trace!("Unit Variant OK");
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        trace!("Newtype");
        if NEWTYPE {
            trace!("Newtype OK");
            seed.deserialize(self.deserializer)
        } else {
            Err(ParseError::InvalidEnumVariant)
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ParseError::InvalidEnumVariant)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(ParseError::InvalidEnumVariant)
    }
}

pub fn print_type_of<T>(_: &T) {
    trace!("{}", core::any::type_name::<T>())
}

#[cfg(test)]
mod deserializer_test {

    use crate::{CmdType, DataType};

    use super::*;
    use core::marker::PhantomData;
    use test_log::test;
    #[test]
    fn deserialize_u16_check_checksum() {
        struct TestVisitor<'de> {
            phantom: PhantomData<&'de ()>,
        }
        impl<'de> TestVisitor<'de> {
            pub fn new() -> TestVisitor<'de> {
                TestVisitor {
                    phantom: PhantomData,
                }
            }
        }
        impl<'de> serde::de::Visitor<'de> for TestVisitor<'de> {
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
        let frame_bytes: [u8; 16] = [
            0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x90, 0x2e, 0x12,
            0x00, 0x05,
        ];
        let mut u16_deserializer = SynDeserializer::new(&frame_bytes);
        for v in frame_bytes.chunks(2) {
            let v: [u8; 2] = v.try_into().unwrap();
            let visitor = TestVisitor::new();
            assert_eq!(
                u16_deserializer.deserialize_u16(visitor),
                Ok(u16::from_be_bytes(v))
            );
        }
        let frame_checksum = u16::from_be_bytes([0x16, 0x8a]);
        assert_eq!(u16_deserializer.checksum, frame_checksum);
    }

    #[test]
    fn deserialize_u32_check_checksum() {
        struct TestVisitor<'de> {
            phantom: PhantomData<&'de ()>,
        }
        impl<'de> TestVisitor<'de> {
            pub fn new() -> TestVisitor<'de> {
                TestVisitor {
                    phantom: PhantomData,
                }
            }
        }
        impl<'de> serde::de::Visitor<'de> for TestVisitor<'de> {
            type Value = u32;
            fn expecting(
                &self,
                _: &mut core::fmt::Formatter<'_>,
            ) -> core::result::Result<(), core::fmt::Error> {
                todo!()
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }
        let frame_bytes: [u8; 16] = [
            0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x90, 0x2e, 0x12,
            0x00, 0x05,
        ];
        let mut u32_deserializer = SynDeserializer::new(&frame_bytes);
        for v in frame_bytes.chunks(4) {
            let v: [u8; 4] = v.try_into().unwrap();
            let visitor = TestVisitor::new();
            assert_eq!(
                u32_deserializer.deserialize_u32(visitor),
                Ok(u32::from_be_bytes(v))
            );
        }
        let frame_checksum = u16::from_be_bytes([0x16, 0x8a]);
        assert_eq!(u32_deserializer.checksum, frame_checksum);
    }

    #[test]
    fn deserialize_enum_checksum() {
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

        struct TestVisitor<'de> {
            phantom: PhantomData<&'de ()>,
        }
        impl<'de> TestVisitor<'de> {
            pub fn new() -> TestVisitor<'de> {
                TestVisitor {
                    phantom: PhantomData,
                }
            }
        }
        impl<'de> serde::de::Visitor<'de> for TestVisitor<'de> {
            type Value = DataType;
            fn expecting(
                &self,
                _: &mut core::fmt::Formatter<'_>,
            ) -> core::result::Result<(), core::fmt::Error> {
                todo!()
            }
            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                trace!("Visit enum");
                let (v, _variant_visitor) = data.variant()?;
                trace!("Exiting Visit enum");
                Ok(v)
            }
        }

        let frame_bytes: [u8; 16] = [
            0xaa, 0x41, 0x00, 0x12, 0x00, 0x3c, 0x48, 0x99, 0x90, 0x9a, 0x00, 0x90, 0x2e, 0x12,
            0x00, 0x05,
        ];
        let mut deserializer = SynDeserializer::new(&frame_bytes);
        for _v in frame_bytes.chunks(2) {
            let visitor = ValueVisitor::new();
            deserializer.deserialize_u16(visitor).unwrap();
        }
        let visitor = TestVisitor::new();
        //Function flow: Visitor, Visitor.Value=DataType
        // Deserializer-> Deserialize enum<Visitor>
        //    Visitor.Visit_Enum(Data: Into_Deserializer for &str (StrDeserializer), with EnumAccess Trait)->V::Value
        //         data.Variant<T>() -> (T,Variant Access), T: DeserializeSeed
        //         data.VariantSeed<T>(seed:PhantomData) -> (T,Variant Access)
        assert_eq!(
            deserializer.deserialize_enum(
                "DataType",
                &["Header", "Cfg1", "Cfg2", "Cfg3", "Data", "Cmd"],
                visitor
            ),
            Ok(DataType::Cmd(CmdType::TurnOffDataFrames))
        );
    }
}
