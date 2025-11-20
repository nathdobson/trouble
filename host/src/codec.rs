//! Opinionated BLE codec
//!
//! Assumes little endian for all types

#[doc(hidden)]
pub trait FixedSize: Sized {
    const SIZE: usize;
}

#[doc(hidden)]
pub trait Type: Sized {
    fn size(&self) -> usize;
}

#[doc(hidden)]
pub trait Encode: Type {
    fn encode(&self, dest: &mut [u8]) -> Result<(), Error>;
}

#[doc(hidden)]
pub trait Decode<'d>: Type {
    fn decode(src: &'d [u8]) -> Result<Self, Error>;
}

#[doc(hidden)]
impl<T: FixedSize> Type for T {
    fn size(&self) -> usize {
        Self::SIZE
    }
}

/// An error encoding a codec
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    /// Insufficient space in the output buffer
    InsufficientSpace,
    /// ???
    InvalidValue,
}
