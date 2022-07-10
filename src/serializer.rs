use crate::{error::SerializeError, Message};
use log::trace;
use serde::{Serialize, Serializer};

pub trait ByteContainer {
    fn enque(&mut self, v: u8) -> Result<(), SerializeError>;
    fn get(&self) -> &[u8];
}

pub struct SynSerializer<B: ByteContainer> {
    bytes: B,
    checksum: u16,
}

impl<B> SynSerializer<B>
where
    B: ByteContainer,
{
    pub fn new(bytes: B) -> SynSerializer<B> {
        SynSerializer {
            bytes,
            //Initial checksum
            checksum: 0xFF_FF,
        }
    }

    fn enque(&mut self, v: u8) -> Result<(), SerializeError> {
        self.bytes.enque(v)?;
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
        Ok(())
    }

    pub fn to_bytes(mut self, m: &Message) -> Result<B, SerializeError> {
        m.serialize(&mut self)?;

        //Add checksum
        self.serialize_end()?;

        Ok(self.bytes)
    }

    fn serialize_end(&mut self) -> Result<(), SerializeError> {
        self.serialize_u16(self.checksum)
    }
}

impl<B> Serializer for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.enque(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        v.to_be_bytes()
            .into_iter()
            .try_fold((), |_, v| self.enque(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        v.to_be_bytes()
            .into_iter()
            .try_fold((), |_, v| self.enque(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        v.to_be_bytes()
            .into_iter()
            .try_fold((), |_, v| self.enque(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.to_be_bytes()
            .into_iter()
            .try_fold((), |_, v| self.enque(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.to_be_bytes()
            .into_iter()
            .try_fold((), |_, v| self.enque(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.enque(v as u8)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.chars()
            .into_iter()
            .try_fold((), |_, v| self.enque(v as u8))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        v.iter().try_fold((), |_, v| self.enque(*v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        trace!("{},{}", name, len);
        //self.serialize_struct(name, len)
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn collect_str<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: std::fmt::Display + ?Sized,
    {
        todo!()
    }
}

impl<B> serde::ser::SerializeSeq for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<B> serde::ser::SerializeMap for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<B> serde::ser::SerializeStruct for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        trace!("{}", key);
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<B> serde::ser::SerializeStructVariant for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<B> serde::ser::SerializeTuple for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<B> serde::ser::SerializeTupleStruct for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<B> serde::ser::SerializeTupleVariant for &mut SynSerializer<B>
where
    B: ByteContainer,
{
    type Ok = ();

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
