#[derive(PartialEq, Debug)]
pub enum ParseError {
    ConfigNeeded,      //Configuration needs to be provided
    TypeRangeOverflow, //Value overflow of allowed range for type
    BaseParseError(BaseParseError),
    Custom,
    IllegalAccess,
    InvalidChecksum,
}

impl serde::de::Error for ParseError {
    fn custom<T>(_msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        todo!()
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
}

#[derive(PartialEq, Debug)]
pub enum SerializeError {
    SpaceExceeded,
    Custom,
}

impl serde::ser::Error for SerializeError {
    fn custom<T>(_msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        todo!()
    }
}

impl serde::ser::StdError for SerializeError {}

impl core::fmt::Display for SerializeError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
