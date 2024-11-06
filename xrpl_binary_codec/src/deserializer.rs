use crate::{
    alloc::{string::String, vec, vec::Vec},
    error::BinaryCodecError,
};
use ascii::AsciiChar;
use bytes::Buf;

use xrpl_types::{
    AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256, IssuedAmount,
    IssuedValue, UInt16, UInt32, UInt64, UInt8,
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
        self.check_remaining(1, "read_u8")?;

        Ok(self.bytes.get_u8())
    }

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, BinaryCodecError> {
        self.check_remaining(len, "read_bytes")?;
        let mut bytes = vec![0; len];
        self.bytes.copy_to_slice(&mut bytes);
        Ok(bytes)
    }

    fn read_array<const LEN: usize>(&mut self) -> Result<[u8; LEN], BinaryCodecError> {
        self.check_remaining(LEN, "read_array")?;
        let mut array = [0; LEN];
        self.bytes.copy_to_slice(&mut array);
        Ok(array)
    }

    fn read_uint8(&mut self) -> Result<UInt8, BinaryCodecError> {
        self.read_u8()
    }

    fn read_uint16(&mut self) -> Result<UInt16, BinaryCodecError> {
        self.check_remaining(2, "read_u16")?;

        Ok(self.bytes.get_u16())
    }

    fn read_uint32(&mut self) -> Result<UInt32, BinaryCodecError> {
        self.check_remaining(4, "read_u32")?;

        Ok(self.bytes.get_u32())
    }

    fn read_uint64(&mut self) -> Result<UInt64, BinaryCodecError> {
        self.check_remaining(8, "read_u64")?;

        Ok(self.bytes.get_u64())
    }

    fn read_h128(&mut self) -> Result<Hash128, BinaryCodecError> {
        self.check_remaining(16, "read_h128")?;

        let mut value = Hash128([0; 16]);
        self.bytes.copy_to_slice(&mut value.0);
        Ok(value)
    }

    fn read_h160(&mut self) -> Result<Hash160, BinaryCodecError> {
        self.check_remaining(20, "read_h160")?;

        let mut value = Hash160([0; 20]);
        self.bytes.copy_to_slice(&mut value.0);
        Ok(value)
    }

    fn read_h256(&mut self) -> Result<Hash256, BinaryCodecError> {
        self.check_remaining(32, "read_h256")?;

        let mut value = Hash256([0; 32]);
        self.bytes.copy_to_slice(&mut value.0);
        Ok(value)
    }

    fn read_blob(&mut self) -> Result<Blob, BinaryCodecError> {
        let count = self.read_vl_prefix()?;
        Ok(Blob(self.read_bytes(count)?))
    }

    /// Read length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    fn read_vl_prefix(&mut self) -> Result<usize, BinaryCodecError> {
        let b1 = self.read_u8()? as usize;
        if b1 <= 192 {
            Ok(b1)
        } else if b1 <= 240 {
            let b2 = self.read_u8()? as usize;
            Ok(193 + (b1 - 193) * 256 + b2)
        } else if b1 <= 254 {
            let b2 = self.read_u8()? as usize;
            let b3 = self.read_u8()? as usize;
            Ok(12481 + (b1 - 241) * 65536 + b2 * 256 + b3)
        } else {
            Err(BinaryCodecError::InvalidLength(
                "Invalid variable length indicator".into(),
            ))
        }
    }

    /// <https://xrpl.org/docs/references/protocol/binary-format#amount-fields>
    fn read_drops_or_issued_value(&mut self) -> Result<DropsOrIssuedValue, BinaryCodecError> {
        const ISSUED_MASK: u64 = 0x8000000000000000;
        const POSITIVE_MASK: u64 = 0x4000000000000000;

        let value = self.read_uint64()?;
        if value & ISSUED_MASK == 0 {
            if value & POSITIVE_MASK == 0 {
                return Err(BinaryCodecError::OutOfRange(
                    "Drops amount should have positive bit set".to_string(),
                ));
            }
            let drops_amount = DropsAmount::from_drops(value ^ POSITIVE_MASK)
                .map_err(|err| BinaryCodecError::OutOfRange(err.to_string()))?;
            Ok(DropsOrIssuedValue::Drops(drops_amount))
        } else {
            if value == ISSUED_MASK {
                return Ok(DropsOrIssuedValue::Issued(IssuedValue::zero()));
            }

            let mantissa =
                (value << 10 >> 10) as i64 * if value & POSITIVE_MASK != 0 { 1 } else { -1 };
            let exponent = (value << 2 >> 56) as i8 - 97;

            let issued_value = IssuedValue::from_mantissa_exponent(mantissa, exponent)
                .map_err(|err| BinaryCodecError::OutOfRange(err.to_string()))?;
            Ok(DropsOrIssuedValue::Issued(issued_value))
        }
    }

    fn read_amount(&mut self) -> Result<Amount, BinaryCodecError> {
        match self.read_drops_or_issued_value()? {
            DropsOrIssuedValue::Drops(drops_amount) => Ok(Amount::Drops(drops_amount)),
            DropsOrIssuedValue::Issued(issued_value) => {
                let currency_code = self.read_currency_code()?;
                let issuer = self.read_account_id_no_length_prefix()?;

                Ok(Amount::Issued(
                    IssuedAmount::from_issued_value(issued_value, currency_code, issuer)
                        .map_err(|err| BinaryCodecError::OutOfRange(err.to_string()))?,
                ))
            }
        }
    }

    /// <https://xrpl.org/docs/references/protocol/binary-format#currency-codes>
    fn read_currency_code(&mut self) -> Result<CurrencyCode, BinaryCodecError> {
        let array = self.read_array::<20>()?;
        if array == [0u8; 20] {
            Ok(CurrencyCode::Xrp)
        } else if array[0] == 0u8 {
            Ok(
                CurrencyCode::standard([ascii(array[12])?, ascii(array[13])?, ascii(array[14])?])
                    .map_err(|err| BinaryCodecError::OutOfRange(err.to_string()))?,
            )
        } else {
            Ok(CurrencyCode::non_standard(array)
                .map_err(|err| BinaryCodecError::OutOfRange(err.to_string()))?)
        }
    }

    fn read_account_id(&mut self) -> Result<AccountId, BinaryCodecError> {
        let len = self.read_vl_prefix()?;
        if len != 20 {
            return Err(BinaryCodecError::OutOfRange(
                "AccountID not 20 bytes".into(),
            ));
        }
        let array = self.read_array()?;
        Ok(AccountId(array))
    }

    fn read_account_id_no_length_prefix(&mut self) -> Result<AccountId, BinaryCodecError> {
        let array = self.read_array()?;
        Ok(AccountId(array))
    }

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

    fn check_remaining(&mut self, len: usize, context: &str) -> Result<(), BinaryCodecError> {
        if self.bytes.remaining() >= len {
            Ok(())
        } else {
            Err(BinaryCodecError::InsufficientBytes(context.into()))
        }
    }
}

#[derive(Debug, PartialEq)]
enum DropsOrIssuedValue {
    Drops(DropsAmount),
    Issued(IssuedValue),
}

fn ascii(byte: u8) -> Result<AsciiChar, BinaryCodecError> {
    AsciiChar::from_ascii(byte)
        .map_err(|err| BinaryCodecError::OutOfRange(format!("Not valid ASCII char: {}", byte)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use xrpl_types::DropsAmount;

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

    #[test]
    fn test_read_blob() {
        let mut s = deserializer(&[3, 0x34, 0x00, 0x12]);
        let value = s.read_blob().unwrap();
        assert_eq!(value, Blob(vec![0x34, 0x00, 0x12]));
    }

    #[test]
    fn test_read_account_id() {
        let mut s = deserializer(&[
            20, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let value = s.read_account_id().unwrap();
        assert_eq!(
            value,
            AccountId([
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
            ])
        );
    }

    #[test]
    fn test_read_account_id_no_length_prefix() {
        let mut s = deserializer(&[
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let value = s.read_account_id_no_length_prefix().unwrap();
        assert_eq!(
            value,
            AccountId([
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
            ])
        );
    }

    /// Tests length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    #[test]
    #[allow(clippy::erasing_op, clippy::identity_op)]
    fn test_read_vl_prefix() {
        // test range 0 to 192
        let mut s = deserializer(&[0, 1, 192]);
        assert_eq!(s.read_vl_prefix().unwrap(), 0);
        assert_eq!(s.read_vl_prefix().unwrap(), 1);
        assert_eq!(s.read_vl_prefix().unwrap(), 192);

        // test range 193 to 12480
        let mut s = deserializer(&[193, 0, 193, 1, 240, 255]);
        assert_eq!(s.read_vl_prefix().unwrap(), 193 + ((193 - 193) * 256) + 0);
        assert_eq!(s.read_vl_prefix().unwrap(), 193 + ((193 - 193) * 256) + 1);
        assert_eq!(193 + ((240 - 193) * 256) + 255, 12480);
        assert_eq!(s.read_vl_prefix().unwrap(), 193 + ((240 - 193) * 256) + 255);

        // test range 12481 to 918744
        let mut s = deserializer(&[241, 0, 0, 241, 0, 1, 241, 1, 0, 241, 255, 255, 254, 212, 23]);
        assert_eq!(
            s.read_vl_prefix().unwrap(),
            12481 + ((241 - 241) * 65536) + (0 * 256) + 0
        );
        assert_eq!(
            s.read_vl_prefix().unwrap(),
            12481 + ((241 - 241) * 65536) + (0 * 256) + 1
        );
        assert_eq!(
            s.read_vl_prefix().unwrap(),
            12481 + ((241 - 241) * 65536) + (1 * 256) + 0
        );
        assert_eq!(
            s.read_vl_prefix().unwrap(),
            12481 + ((241 - 241) * 65536) + (255 * 256) + 255
        );
        assert_eq!(12481 + ((254 - 241) * 65536) + (212 * 256) + 23, 918744);
        assert_eq!(
            s.read_vl_prefix().unwrap(),
            12481 + ((254 - 241) * 65536) + (212 * 256) + 23
        );

        // test out of range
        let mut s = deserializer(&[255]);
        let result = s.read_vl_prefix();
        assert_matches!(result, Err(BinaryCodecError::InvalidLength(_)));
    }

    #[test]
    fn test_read_currency_code_xrp() {
        let mut s = deserializer(&[0u8; 20]);
        let value = s.read_currency_code().unwrap();
        assert_eq!(value, CurrencyCode::xrp());
    }

    #[test]
    fn test_read_currency_code_standard() {
        let bytes = [
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            AsciiChar::U.as_byte(),
            AsciiChar::S.as_byte(),
            AsciiChar::D.as_byte(),
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let mut s = deserializer(&bytes);
        let value = s.read_currency_code().unwrap();
        assert_eq!(
            value,
            CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap()
        );
    }

    #[test]
    fn test_read_currency_code_non_standard() {
        let mut s = deserializer(&[
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ]);
        let value = s.read_currency_code().unwrap();
        assert_eq!(
            value,
            CurrencyCode::non_standard([
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_read_drops_amount() {
        let mut s = deserializer(&[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
        let value = s.read_drops_or_issued_value().unwrap();
        assert_eq!(
            value,
            DropsOrIssuedValue::Drops(DropsAmount::from_drops(10_000).unwrap())
        );
    }

    #[test]
    fn test_read_issued_value_zero() {
        let mut s = deserializer(&[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let value = s.read_drops_or_issued_value().unwrap();
        assert_eq!(value, DropsOrIssuedValue::Issued(IssuedValue::zero()));
    }

    #[test]
    fn test_read_issued_value_positive() {
        let mut s = deserializer(&[0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00]);
        let value = s.read_drops_or_issued_value().unwrap();
        assert_eq!(
            value,
            DropsOrIssuedValue::Issued(
                IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap()
            )
        );
    }

    #[test]
    fn test_read_issued_value_negative() {
        let mut s = deserializer(&[0x95, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00]);
        let value = s.read_drops_or_issued_value().unwrap();
        assert_eq!(
            value,
            DropsOrIssuedValue::Issued(
                IssuedValue::from_mantissa_exponent(-1_000_000_000_000_000, -10).unwrap()
            )
        );
    }

    #[test]
    fn test_read_amount_drop() {
        let mut s = deserializer(&[0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
        let value = s.read_amount().unwrap();
        assert_eq!(value, Amount::drops(10_000).unwrap());
    }

    #[test]
    fn test_read_amount_issued() {
        let mut s = deserializer(&[
            0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        let currency = CurrencyCode::non_standard([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        let issuer = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let expected_amount = Amount::issued(value, currency, issuer).unwrap();
        let amount = s.read_amount().unwrap();
        assert_eq!(amount, expected_amount);
    }
}
