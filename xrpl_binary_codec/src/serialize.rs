use crate::serializer::Serializer;
use xrpl_types::serialize::Serialize;
use crate::error::BinaryCodecError;

pub fn serialize(object: impl Serialize) -> Result<Vec<u8>, BinaryCodecError> {
    let mut s = Serializer::new();
    object.serialize(&mut s)?;
    s.into_bytes()
}
