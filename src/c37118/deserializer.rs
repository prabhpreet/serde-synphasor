use super::error::*;
use super::message::MAX_EXTENDED_FRAME_SIZE;
use crate::Container;
use log::trace;
use serde::{
    de::{EnumAccess, SeqAccess, VariantAccess},
    Deserialize, Deserializer,
};

pub struct SynDeserializer<'de> {
    bytes: &'de [u8],
    index: usize,
    checksum: u16,
}

impl<'de> SynDeserializer<'de> {
    pub fn new(bytes: &'de [u8]) -> SynDeserializer<'de> {
        SynDeserializer {
            bytes,
            index: 0,
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
        trace!("Deserialize bytes");
        let index = self.index;
        let size = self.bytes.len() - index;
        let bytes = &self.bytes[index..];
        self.enque_checksum(&bytes);
        self.index += size;

        trace!("Visit bytes");
        visitor.visit_borrowed_bytes(&bytes)
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
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

        visitor.visit_enum(SynDeserializerEA::new(self))
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

struct SynDeserializerEA<'a, 'de: 'a> {
    deserializer: &'a mut SynDeserializer<'de>,
}

impl<'a, 'de> SynDeserializerEA<'a, 'de> {
    fn new(deserializer: &'a mut SynDeserializer<'de>) -> Self {
        trace!("EA Initialized");
        SynDeserializerEA { deserializer }
    }
}

impl<'a, 'de> EnumAccess<'de> for SynDeserializerEA<'a, 'de> {
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

impl<'a, 'de> VariantAccess<'de> for SynDeserializerEA<'a, 'de> {
    type Error = ParseError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        trace!("Unit Variant");
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        trace!("Newtype");
        seed.deserialize(self.deserializer)
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

    use crate::{create_phantom_container, PhantomContainer};

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
}
