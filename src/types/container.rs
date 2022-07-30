use core::{fmt::Debug, marker::PhantomData};
use serde::Serialize;

use crate::c37118::message::CmdStore;

#[derive(PartialEq, Debug)]
pub enum ContainerError {
    SpaceExceeded,
}

/// Generic Container for buffered storage
pub trait Container<T, const MAX_SIZE: usize>: Debug
where
    T: Sized + PartialEq,
{
    fn enque(&mut self, v: T) -> Result<(), ContainerError>;
    fn get(&self) -> &[T];
}

#[derive(Debug, PartialEq)]
pub struct PhantomContainer<T, const MAX_SIZE: usize> {
    phantom_data: ::core::marker::PhantomData<T>,
}

impl<T, const MAX_SIZE: usize> Container<T, MAX_SIZE> for PhantomContainer<T, MAX_SIZE>
where
    T: Sized + PartialEq + Debug,
{
    fn enque(&mut self, _v: T) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get(&self) -> &[T] {
        &[]
    }
}

pub fn create_phantom_container<T, const MAX_SIZE: usize>() -> PhantomContainer<T, MAX_SIZE> {
    PhantomContainer {
        phantom_data: PhantomData,
    }
}
