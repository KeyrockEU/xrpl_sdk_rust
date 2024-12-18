use core::fmt;
use core::fmt::Display;
use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt64, UInt8};

pub trait SerError: fmt::Debug + fmt::Display + Sized {
    fn unimplemented(msg: impl Display) -> Self;
}

/// Serializes XRPL objects to a [`Serializer`]
pub trait Serialize {
    /// Serialize the object
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

/// Serialize for XRPL types and objects
pub trait Serializer {
    type Error: SerError;
    type ArraySerializer<'a>: ArraySerializer<Error = Self::Error>
    where
        Self: 'a;

    fn serialize_account_id(
        &mut self,
        field_name: &str,
        account_id: AccountId,
    ) -> Result<(), Self::Error>;

    fn serialize_amount(&mut self, field_name: &str, amount: Amount) -> Result<(), Self::Error>;

    fn serialize_blob(&mut self, field_name: &str, blob: &Blob) -> Result<(), Self::Error>;

    fn serialize_hash128(&mut self, field_name: &str, hash128: Hash128) -> Result<(), Self::Error>;

    fn serialize_hash160(&mut self, field_name: &str, hash160: Hash160) -> Result<(), Self::Error>;

    fn serialize_hash256(&mut self, field_name: &str, hash256: Hash256) -> Result<(), Self::Error>;

    fn serialize_uint8(&mut self, field_name: &str, uint8: UInt8) -> Result<(), Self::Error>;

    fn serialize_uint16(&mut self, field_name: &str, uint16: UInt16) -> Result<(), Self::Error>;

    fn serialize_uint32(&mut self, field_name: &str, uint32: UInt32) -> Result<(), Self::Error>;

    fn serialize_uint64(&mut self, field_name: &str, uint64: UInt64) -> Result<(), Self::Error>;

    fn serialize_array(
        &mut self,
        field_name: &str,
    ) -> Result<Self::ArraySerializer<'_>, Self::Error>;
}

pub trait ArraySerializer {
    type Error: fmt::Debug + fmt::Display;

    fn serialize_object<T: Serialize>(
        &mut self,
        field_name: &str,
        object: &T,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<(), Self::Error>;
}
