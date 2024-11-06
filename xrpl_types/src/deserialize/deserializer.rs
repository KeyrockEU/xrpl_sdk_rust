use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt8, UInt64};
use core::fmt;
use crate::deserialize::Deserialize;

/// Serialize for XRPL types and objects
pub trait Deserializer {
    type Error: fmt::Debug + fmt::Display;

    fn deserialize_account_id(&mut self, field_name: &str) -> Result<AccountId, Self::Error>;

    fn deserialize_amount(&mut self, field_name: &str) -> Result<Amount, Self::Error>;

    fn deserialize_blob(&mut self, field_name: &str) -> Result<Blob, Self::Error>;

    fn deserialize_hash128(&mut self, field_name: &str) -> Result<Hash128, Self::Error>;

    fn deserialize_hash160(&mut self, field_name: &str) -> Result<Hash160, Self::Error>;

    fn deserialize_hash256(&mut self, field_name: &str) -> Result<Hash256, Self::Error>;

    fn deserialize_uint8(&mut self, field_name: &str) -> Result<UInt8, Self::Error>;

    fn deserialize_uint16(&mut self, field_name: &str) -> Result<UInt16, Self::Error>;

    fn deserialize_uint32(&mut self, field_name: &str) -> Result<UInt32, Self::Error>;

    fn deserialize_uint64(&mut self, field_name: &str) -> Result<UInt64, Self::Error>;

    fn deserialize_array<T: Deserialize>(&mut self, array_field_name: &str, object_field_name: &str) -> Result<Vec<T>, Self::Error>;
}
