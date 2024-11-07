use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt64, UInt8};
use core::fmt;
use core::fmt::Display;

pub trait DeserError: fmt::Debug + fmt::Display + Sized {
    fn missing_field(field: &str) -> Self;
    fn invalid_value(msg: impl Display) -> Self;

    fn unwrap_field_value<T>(field: &str, value: Option<T>) -> Result<T, Self> {
        match value {
            None => Err(Self::missing_field(field)),
            Some(value) => Ok(value),
        }
    }
}

/// Deserializes XRPL objects to a [`Deserializer`]
pub trait Deserialize {
    /// Deserialize the object
    fn deserialize<S: Deserializer>(&self, deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized;
}

/// Deserializer for XRPL types and objects
pub trait Deserializer {
    type Error: DeserError;

    /// Visit all fields as they are deserialized
    fn deserialize<V: Visitor>(self, visitor: &mut V) -> Result<(), Self::Error>;

    /// Deserialize single field in order
    fn deserialize_single_field(&mut self, field_name: &str) -> Result<impl FieldAccessor, Self::Error>;

    // fn deserialize_array<T: Deserialize>(&mut self, array_field_name: &str, object_field_name: &str) -> Result<Vec<T>, Self::Error>;
}

pub trait Visitor {
    fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
        &mut self,
        field_name: &str,
        field_accessor: F,
    ) -> Result<(), E>;
}

pub trait FieldAccessor {
    type Error: DeserError;

    fn deserialize_account_id(&mut self) -> Result<AccountId, Self::Error>;

    fn deserialize_amount(&mut self) -> Result<Amount, Self::Error>;

    fn deserialize_blob(&mut self) -> Result<Blob, Self::Error>;

    fn deserialize_hash128(&mut self) -> Result<Hash128, Self::Error>;

    fn deserialize_hash160(&mut self) -> Result<Hash160, Self::Error>;

    fn deserialize_hash256(&mut self) -> Result<Hash256, Self::Error>;

    fn deserialize_uint8(&mut self) -> Result<UInt8, Self::Error>;

    fn deserialize_uint16(&mut self) -> Result<UInt16, Self::Error>;

    fn deserialize_uint32(&mut self) -> Result<UInt32, Self::Error>;

    fn deserialize_uint64(&mut self) -> Result<UInt64, Self::Error>;
}
