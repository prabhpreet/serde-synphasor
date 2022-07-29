use crate::{ContainerError, TimeError};
use log::error;
#[derive(PartialEq, Debug)]
pub enum ParseError {
    ConfigNeeded,      //Configuration needs to be provided
    TypeRangeOverflow, //Value overflow of allowed range for type
    BaseFrame(BaseParseError),
    Custom,
    IllegalAccess,
    InvalidFrameSize,
    InvalidChecksum,
    InvalidEnumVariant,
    BytesExceedFrameSize,
    ContainerError(ContainerError),
}

impl serde::de::Error for ParseError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        error!("{}", msg);
        ParseError::Custom
    }
}

impl serde::de::StdError for ParseError {}

impl core::fmt::Display for ParseError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

#[derive(PartialEq, Debug)]
pub enum BaseParseError {
    IncorrectSyncWord, // Sync word is not 0xAA
    IncorrectReservedSyncBit,
    UnknownFrameVersionNumber, // Unknown standard version number
    IncorrectReservedFracsecBit,
    UnknownTimeQuality,
    UnknownFrameType,
    Fracsec(TimeError),
}

#[derive(PartialEq, Debug)]
pub enum SerializeError {
    Custom,
    ContainerError(ContainerError),
}

impl serde::ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        error!("{}", msg);
        SerializeError::Custom
    }
}

impl serde::ser::StdError for SerializeError {}

impl core::fmt::Display for SerializeError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
