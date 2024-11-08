use crate::deserializer::Deserializer;
use crate::BinaryCodecError;
use xrpl_types::deserialize::Deserialize;

/// Deserializes the given bytes in the canonical binary format <https://xrpl.org/serialization.html> to `T`
pub fn deserialize<T: Deserialize>(bytes: &[u8]) -> Result<T, BinaryCodecError> {
    let d = Deserializer::new(bytes);
    T::deserialize(d)
}
