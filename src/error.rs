use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    ConfigNeeded,      //Configuration needs to be provided
    TypeRangeOverflow, //Value overflow of allowed range for type
    BaseParseError(BaseParseError),
    Custom,
    IllegalAccess,
    InvalidChecksum,
}

#[derive(PartialEq, Debug)]
pub enum BaseParseError {
    IncorrectSyncWord, // Sync word is not 0xAA
    IncorrectReservedSyncBit,
    UnknownFrameVersionNumber, // Unknown standard version number
    IncorrectReservedFracsecBit,
    UnknownTimeQuality,
    UnknownFrameType,
}

impl Display for ParseError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for BaseParseError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for ParseError {}
impl serde::de::Error for ParseError {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        ParseError::Custom
    }
}

#[derive(PartialEq, Debug)]
pub enum SerializeError {
    SpaceExceeded,
    Custom,
}
impl Display for SerializeError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for SerializeError {}
impl serde::ser::Error for SerializeError {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializeError::Custom
    }
}
