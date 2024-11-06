use crate::alloc::{string::String, vec, vec::Vec};
use crate::error::BinaryCodecError;
use bytes::Buf;

use xrpl_types::{
    AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt64, UInt8,
};

use xrpl_types::deserialize::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Deserializer<B> {
    bytes: B,
}

impl<B: Buf> xrpl_types::deserialize::Deserializer for Deserializer<B> {
    type Error = BinaryCodecError;

    fn deserialize_account_id(&mut self, field_name: &str) -> Result<AccountId, BinaryCodecError> {
        todo!()
    }

    fn deserialize_amount(&mut self, field_name: &str) -> Result<Amount, BinaryCodecError> {
        todo!()
    }

    fn deserialize_blob(&mut self, field_name: &str) -> Result<Blob, BinaryCodecError> {
        todo!()
    }

    fn deserialize_hash128(&mut self, field_name: &str) -> Result<Hash128, BinaryCodecError> {
        todo!()
    }

    fn deserialize_hash160(&mut self, field_name: &str) -> Result<Hash160, BinaryCodecError> {
        todo!()
    }

    fn deserialize_hash256(&mut self, field_name: &str) -> Result<Hash256, BinaryCodecError> {
        todo!()
    }

    fn deserialize_uint8(&mut self, field_name: &str) -> Result<UInt8, BinaryCodecError> {
        todo!()
    }

    fn deserialize_uint16(&mut self, field_name: &str) -> Result<UInt16, BinaryCodecError> {
        todo!()
    }

    fn deserialize_uint32(&mut self, field_name: &str) -> Result<UInt32, BinaryCodecError> {
        todo!()
    }

    fn deserialize_uint64(&mut self, field_name: &str) -> Result<UInt64, BinaryCodecError> {
        todo!()
    }

    fn deserialize_array<T: Deserialize>(
        &mut self,
        array_field_name: &str,
        object_field_name: &str,
    ) -> Result<Vec<T>, Self::Error> {
        todo!()
    }

    // fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), BinaryCodecError> {
    //     if self.bytes.remaining() < buf.len() {
    //         return Err(BinaryCodecError::InsufficientBytes("read_exact".into()));
    //     }
    //     self.bytes.copy_to_slice(buf);
    //     Ok(())
    // }
    //
    // fn deserialize_account_id(&mut self) -> Result<AccountId, BinaryCodecError> {
    //     let mut bytes = [0u8; 20];
    //     self.read_exact(&mut bytes)?;
    //     Ok(AccountId(bytes))
    // }
    //
    // fn deserialize_amount(&mut self) -> Result<Amount, BinaryCodecError> {
    //     unimplemented!()
    // }
    //
    // fn deserialize_blob(&mut self, len: usize) -> Result<Blob, BinaryCodecError> {
    //     let mut bytes = vec![0u8; len];
    //     self.read_exact(&mut bytes)?;
    //     Ok(Blob(bytes))
    // }
    //
    // fn deserialize_hash128(&mut self) -> Result<Hash128, BinaryCodecError> {
    //     let mut bytes = [0u8; 16];
    //     self.read_exact(&mut bytes)?;
    //     Ok(Hash128(bytes))
    // }
    //
    // fn deserialize_hash160(&mut self) -> Result<Hash160, BinaryCodecError> {
    //     let mut bytes = [0u8; 20];
    //     self.read_exact(&mut bytes)?;
    //     Ok(Hash160(bytes))
    // }
    //
    // fn deserialize_hash256(&mut self) -> Result<Hash256, BinaryCodecError> {
    //     let mut bytes = [0u8; 32];
    //     self.read_exact(&mut bytes)?;
    //     Ok(Hash256(bytes))
    // }
    //
    // fn deserialize_uint8(&mut self) -> Result<UInt8, BinaryCodecError> {
    //     let mut bytes = [0u8; 1];
    //     self.read_exact(&mut bytes)?;
    //     Ok(UInt8::from_be_bytes(bytes))
    // }
    //
    // fn deserialize_uint16(&mut self) -> Result<UInt16, BinaryCodecError> {
    //     let mut bytes = [0u8; 2];
    //     self.read_exact(&mut bytes)?;
    //     Ok(UInt16::from_be_bytes(bytes))
    // }
    //
    // fn deserialize_uint32(&mut self) -> Result<UInt32, BinaryCodecError> {
    //     let mut bytes = [0u8; 4];
    //     self.read_exact(&mut bytes)?;
    //     Ok(UInt32::from_be_bytes(bytes))
    // }
    //
    // fn deserialize_uint64(&mut self) -> Result<Uint64, BinaryCodecError> {
    //     let mut bytes = [0u8; 8];
    //     self.read_exact(&mut bytes)?;
    //     Ok(Uint64::from_be_bytes(bytes))
    // }
    //
}

impl<B: Buf> Deserializer<B> {
    pub fn new(bytes: B) -> Self {
        Self { bytes }
    }

    fn read_u8(&mut self) -> Result<u8, BinaryCodecError> {
        if self.bytes.remaining() >= 1 {
            Ok(self.bytes.get_u8())
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_u8".into()))
        }
    }

    fn read_uint8(&mut self) -> Result<UInt8, BinaryCodecError> {
        self.read_u8()
    }

    fn read_uint16(&mut self) -> Result<UInt16, BinaryCodecError> {
        if self.bytes.remaining() >= 2 {
            Ok(self.bytes.get_u16())
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_u16".into()))
        }
    }

    fn read_uint32(&mut self) -> Result<UInt32, BinaryCodecError> {
        if self.bytes.remaining() >= 4 {
            Ok(self.bytes.get_u32())
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_u32".into()))
        }
    }

    fn read_uint64(&mut self) -> Result<UInt64, BinaryCodecError> {
        if self.bytes.remaining() >= 8 {
            Ok(self.bytes.get_u64())
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_u64".into()))
        }
    }

    fn read_h128(&mut self) -> Result<Hash128, BinaryCodecError> {
        if self.bytes.remaining() >= 16 {
            let mut value = Hash128([0; 16]);
            self.bytes.copy_to_slice(&mut value.0);
            Ok(value)
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_h128".into()))
        }
    }

    fn read_h160(&mut self) -> Result<Hash160, BinaryCodecError> {
        if self.bytes.remaining() >= 20 {
            let mut value = Hash160([0; 20]);
            self.bytes.copy_to_slice(&mut value.0);
            Ok(value)
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_h160".into()))
        }
    }

    fn read_h256(&mut self) -> Result<Hash256, BinaryCodecError> {
        if self.bytes.remaining() >= 32 {
            let mut value = Hash256([0; 32]);
            self.bytes.copy_to_slice(&mut value.0);
            Ok(value)
        } else {
            Err(BinaryCodecError::InsufficientBytes("read_h256".into()))
        }
    }

    // fn read_variable_length(&mut self) -> Result<usize, BinaryCodecError> {
    //     let b1 = self.read_u8()? as usize;
    //     if b1 <= 192 {
    //         Ok(b1)
    //     } else if b1 <= 240 {
    //         let b2 = self.read_u8()? as usize;
    //         Ok(193 + (b1 - 193) * 256 + b2)
    //     } else if b1 <= 254 {
    //         let b2 = self.read_u8()? as usize;
    //         let b3 = self.read_u8()? as usize;
    //         Ok(12481 + (b1 - 241) * 65536 + b2 * 256 + b3)
    //     } else {
    //         Err(BinaryCodecError::InvalidLength(
    //             "Invalid variable length indicator".into(),
    //         ))
    //     }
    // }
    //
    // fn read_field_ordinal(&mut self) -> Result<u32, BinaryCodecError> {
    //     let mut type_code = self.read_u8()? as u32;
    //     let mut nth = type_code & 15;
    //     type_code >>= 4;
    //     if type_code == 0 {
    //         type_code = self.read_u8()? as u32;
    //         if type_code == 0 || type_code < 16 {
    //             return Err(BinaryCodecError::OutOfRange(
    //                 "FieldOrdinal, type_code out of range".into(),
    //             ));
    //         }
    //     }
    //     if nth == 0 {
    //         nth = self.read_u8()? as u32;
    //         if nth == 0 || nth < 16 {
    //             return Err(BinaryCodecError::OutOfRange(
    //                 "FieldOrdinal, type_code out of range".into(),
    //             ));
    //         }
    //     }
    //     Ok((type_code << 16) | nth)
    // }

    // fn read_field(&mut self) -> Result<FieldInstance, BinaryCodecError> {
    //     let ordinal = self.read_field_ordinal()?;
    //     self.field_ordinal_lookup
    //         .get(&ordinal)
    //         .cloned()
    //         .ok_or(BinaryCodecError::FieldNotFound("Field not found".into()))
    // }
    //
    // fn read_field_value(&mut self, info: &FieldInfo) -> Result<Vec<u8>, BinaryCodecError> {
    //     let size_hint: Option<usize> = if info.is_vl_encoded {
    //         Some(self.read_variable_length()?)
    //     } else {
    //         None
    //     };
    //     let bytes = match info.field_type {
    //         TypeCode::Hash256 => self.deserialize_hash256()?.0.to_vec(),
    //         TypeCode::AccountId => self.deserialize_account_id()?.0.to_vec(),
    //         TypeCode::Blob => {
    //             let hint =
    //                 size_hint.ok_or(BinaryCodecError::FieldNotFound("missing hint".into()))?;
    //             self.deserialize_blob(hint)?.0.to_vec()
    //         }
    //         TypeCode::Object => self.deserialize_object()?,
    //         TypeCode::Array => self.deserialize_array()?,
    //         _ => vec![], // TODO: default other types to Blob for now
    //     };
    //     Ok(bytes)
    // }

    // pub fn end(&mut self) -> bool {
    //     self.bytes.remaining() == 0
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deserializer(bytes: &[u8]) -> Deserializer<&[u8]> {
        Deserializer::new(bytes)
    }

    #[test]
    fn test_read_uint8() {
        let mut s = deserializer(&[0x12]);
        let value = s.read_uint8().unwrap();
        assert_eq!(value, 0x12u8);
    }

    #[test]
    fn test_read_uint16() {
        let mut s = deserializer(&[0x34, 0x12]);
        let value = s.read_uint16().unwrap();
        assert_eq!(value, 0x12 + (0x34 << 8));
    }

    #[test]
    fn test_read_uint32() {
        let mut s = deserializer(&[0x34, 0x00, 0x00, 0x12]);
        let value = s.read_uint32().unwrap();
        assert_eq!(value, 0x12 + (0x34 << 24));
    }

    #[test]
    fn test_read_uint64() {
        let mut s = deserializer(&[0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12]);
        let value = s.read_uint64().unwrap();
        assert_eq!(value, 0x12 + (0x34 << 56));
    }

    #[test]
    fn test_read_h128() {
        let mut s = deserializer(&[
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x12,
        ]);
        let value = s.read_h128().unwrap();
        assert_eq!(
            value,
            Hash128([
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x12,
            ])
        );
    }

    #[test]
    fn test_read_h160() {
        let mut s = deserializer(&[
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let value = s.read_h160().unwrap();
        assert_eq!(
            value,
            Hash160([
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
            ])
        );
    }

    #[test]
    fn test_read_h256() {
        let mut s = deserializer(&[
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x12,
        ]);
        let value = s.read_h256().unwrap();
        assert_eq!(
            value,
            Hash256([
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x12,
            ])
        );
    }
}
