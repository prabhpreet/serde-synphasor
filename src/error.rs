use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    ConfigNeeded,
    TypeRangeOverflow,
}

#[derive(Debug)]
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
