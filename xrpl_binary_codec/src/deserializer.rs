use crate::{
    alloc::{format, vec, vec::Vec},
    error::BinaryCodecError,
};
use ascii::AsciiChar;
use bytes::Buf;
use core::fmt::Display;
use xrpl_types::{
    deserialize, AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256,
    IssuedAmount, IssuedValue, UInt16, UInt32, UInt64, UInt8,
};

use crate::alloc::string::ToString;
use crate::field::{field_info, FieldCode, FieldId, TypeCode};
use xrpl_types::deserialize::{DeserError, Deserialize, Visitor};

#[derive(Debug, Clone, Default)]
pub struct Deserializer<B> {
    bytes: B,
    object_deserializer: bool,
    previous_field_id: Option<FieldId>,
}

impl DeserError for BinaryCodecError {
    fn missing_field(field: &str) -> Self {
        BinaryCodecError::MissingField(field.to_string())
    }

    fn unexpected_field(field: &str) -> Self {
        BinaryCodecError::InvalidField(format!("Unexpected field: {}", field))
    }

    fn invalid_value(msg: impl Display) -> Self {
        BinaryCodecError::OutOfRange(msg.to_string())
    }
}

impl<B: Buf> deserialize::Deserializer for Deserializer<B> {
    type Error = BinaryCodecError;

    fn deserialize<V: Visitor>(mut self, visitor: &mut V) -> Result<(), Self::Error> {
        loop {
            if self.bytes.remaining() == 0 {
                return Ok(());
            }

            let field_id = self.read_field_id()?;
            let field_name = get_field_name(field_id)?;

            if field_id == FieldId::from_type_field(TypeCode::Object, FieldCode(1))
                && self.object_deserializer
            {
                return Ok(());
            }

            self.set_and_check_field_order(field_id)?;

            if field_id.type_code == TypeCode::Array {
                let array_deserializer = ArrayDeserializer {
                    deserializer: &mut self,
                };
                visitor.visit_array(field_name, array_deserializer)?;
            } else {
                visitor.visit_field(
                    field_name,
                    FieldAccessor {
                        deserializer: &mut self,
                        type_code: field_id.type_code,
                    },
                )?;
            }
        }
    }

    fn deserialize_single_field(
        &mut self,
        expected_field_name: &str,
    ) -> Result<impl deserialize::FieldAccessor<Error = BinaryCodecError>, Self::Error> {
        let field_id = self.read_field_id()?;
        let field_name = get_field_name(field_id)?;
        self.set_and_check_field_order(field_id)?;

        if field_name != expected_field_name {
            return Err(BinaryCodecError::InvalidField(format!(
                "Expected field {}, found {}",
                expected_field_name, field_name
            )));
        }

        Ok(FieldAccessor {
            deserializer: self,
            type_code: field_id.type_code,
        })
    }
}

#[derive(Debug)]
pub struct ArrayDeserializer<'a, B> {
    deserializer: &'a mut Deserializer<B>,
}

impl<'a, B: Buf> deserialize::ArrayDeserializer for ArrayDeserializer<'a, B> {
    type Error = BinaryCodecError;

    fn deserialize_object<T: Deserialize>(
        &mut self,
        expected_field_name: &str,
    ) -> Result<Option<T>, Self::Error> {
        let field_id = self.deserializer.read_field_id()?;

        if field_id == FieldId::from_type_field(TypeCode::Array, FieldCode(1)) {
            return Ok(None);
        }

        if field_id.type_code != TypeCode::Object {
            return Err(BinaryCodecError::InvalidField(format!(
                "Expected object type, found {}",
                field_id.type_code
            )));
        }

        let field_name = get_field_name(field_id)?;

        if field_name == expected_field_name {
            let object_deserializer = Deserializer {
                bytes: &mut self.deserializer.bytes,
                object_deserializer: true,
                previous_field_id: None,
            };

            let object = T::deserialize(object_deserializer)?;

            Ok(Some(object))
        } else {
            Err(BinaryCodecError::InvalidField(format!(
                "Expected field {}, found {}",
                expected_field_name, field_name
            )))
        }
    }
}

#[derive(Debug)]
struct FieldAccessor<'a, B> {
    deserializer: &'a mut Deserializer<B>,
    type_code: TypeCode,
}

impl<'a, B> FieldAccessor<'a, B> {
    fn check_type(&self, expected_type_code: TypeCode) -> Result<(), BinaryCodecError> {
        if self.type_code != expected_type_code {
            Err(BinaryCodecError::InvalidField(format!(
                "Expected type {}, found {}",
                expected_type_code, self.type_code
            )))
        } else {
            Ok(())
        }
    }
}

impl<'a, B: Buf> deserialize::FieldAccessor for FieldAccessor<'a, B> {
    type Error = BinaryCodecError;

    fn deserialize_account_id(self) -> Result<AccountId, Self::Error> {
        self.check_type(TypeCode::AccountId)?;
        self.deserializer.read_account_id()
    }

    fn deserialize_amount(self) -> Result<Amount, Self::Error> {
        self.check_type(TypeCode::Amount)?;
        self.deserializer.read_amount()
    }

    fn deserialize_blob(self) -> Result<Blob, Self::Error> {
        self.check_type(TypeCode::Blob)?;
        self.deserializer.read_blob()
    }

    fn deserialize_hash128(self) -> Result<Hash128, Self::Error> {
        self.check_type(TypeCode::Hash128)?;
        self.deserializer.read_h128()
    }

    fn deserialize_hash160(self) -> Result<Hash160, Self::Error> {
        self.check_type(TypeCode::Hash160)?;
        self.deserializer.read_h160()
    }

    fn deserialize_hash256(self) -> Result<Hash256, Self::Error> {
        self.check_type(TypeCode::Hash256)?;
        self.deserializer.read_h256()
    }

    fn deserialize_uint8(self) -> Result<UInt8, Self::Error> {
        self.check_type(TypeCode::UInt8)?;
        self.deserializer.read_uint8()
    }

    fn deserialize_uint16(self) -> Result<UInt16, Self::Error> {
        self.check_type(TypeCode::UInt16)?;
        self.deserializer.read_uint16()
    }

    fn deserialize_uint32(self) -> Result<UInt32, Self::Error> {
        self.check_type(TypeCode::UInt32)?;
        self.deserializer.read_uint32()
    }

    fn deserialize_uint64(self) -> Result<UInt64, Self::Error> {
        self.check_type(TypeCode::UInt64)?;
        self.deserializer.read_uint64()
    }
}

impl<B: Buf> Deserializer<B> {
    pub fn new(bytes: B) -> Self {
        Self {
            bytes,
            object_deserializer: false,
            previous_field_id: None,
        }
    }

    fn set_and_check_field_order(&mut self, new_field_id: FieldId) -> Result<(), BinaryCodecError> {
        if let Some(previous_field_id) = self.previous_field_id {
            if previous_field_id == new_field_id {
                return Err(BinaryCodecError::FieldOrder(format!(
                    "Field appears twice: {:?}",
                    new_field_id
                )));
            }
            if previous_field_id > new_field_id {
                return Err(BinaryCodecError::FieldOrder(format!(
                    "Field out of order: {:?}",
                    new_field_id
                )));
            }
        }
        self.previous_field_id = Some(new_field_id);

        Ok(())
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

    /// <https://xrpl.org/docs/references/protocol/binary-format#field-ids>
    fn read_field_id(&mut self) -> Result<FieldId, BinaryCodecError> {
        let byte = self.read_u8()?;
        let type_code = byte >> 4;
        let field_code = byte & 0b1111;

        let type_code = if type_code == 0 {
            self.read_u8()?
        } else {
            type_code
        };

        let field_code = if field_code == 0 {
            self.read_u8()?
        } else {
            field_code
        };

        let type_code = TypeCode::from_discriminant_opt(type_code).ok_or_else(|| {
            BinaryCodecError::OutOfRange(format!("Unknown type code: {}", type_code))
        })?;
        let field_code = FieldCode(field_code);
        Ok(FieldId::from_type_field(type_code, field_code))
    }

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
        .map_err(|_err| BinaryCodecError::OutOfRange(format!("Not valid ASCII char: {}", byte)))
}

pub fn get_field_name(field_id: FieldId) -> Result<&'static str, BinaryCodecError> {
    field_info::field_name_by_id(field_id).ok_or_else(|| {
        BinaryCodecError::InvalidField(format!("Field with id {:?} is not known", field_id))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{FieldCode, TypeCode};
    use crate::serialize;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use enumflags2::BitFlags;
    use xrpl_types::deserialize::{Deserialize, Deserializer, FieldAccessor};
    use xrpl_types::{
        AccountSetTransaction, DropsAmount, Memo, OfferCreateTransaction, Transaction,
    };

    fn deserializer(bytes: &[u8]) -> super::Deserializer<&[u8]> {
        super::Deserializer::new(bytes)
    }

    struct TestObject {
        field1: UInt32,
        field2: UInt32,
    }

    impl Deserialize for TestObject {
        fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
        where
            Self: Sized,
        {
            #[derive(Default)]
            struct Visitor {
                field1: Option<UInt32>,
                field2: Option<UInt32>,
            }

            impl deserialize::Visitor for Visitor {
                fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                    &mut self,
                    field_name: &str,
                    field_accessor: F,
                ) -> Result<(), E> {
                    match field_name {
                        "NetworkID" => {
                            self.field1 = Some(field_accessor.deserialize_uint32()?);
                        }
                        "Flags" => {
                            self.field2 = Some(field_accessor.deserialize_uint32()?);
                        }
                        _ => (),
                    }
                    Ok(())
                }

                fn visit_array<E: DeserError, AD: deserialize::ArrayDeserializer<Error = E>>(
                    &mut self,
                    _field_name: &str,
                    _array_deserializer: AD,
                ) -> Result<(), E> {
                    Ok(())
                }
            }

            let mut visitor = Visitor::default();

            deserializer.deserialize(&mut visitor)?;

            Ok(TestObject {
                field1: S::Error::unwrap_field_value("NetworkID", visitor.field1)?,
                field2: S::Error::unwrap_field_value("Flags", visitor.field2)?,
            })
        }
    }

    #[derive(Default)]
    pub struct ObjectWithArrayVisitor {
        objects: Vec<TestObject>,
        tick_size: Option<UInt8>,
    }

    impl deserialize::Visitor for ObjectWithArrayVisitor {
        fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
            &mut self,
            field_name: &str,
            field_accessor: F,
        ) -> Result<(), E> {
            #[allow(clippy::single_match)]
            match field_name {
                "TickSize" => {
                    self.tick_size = Some(field_accessor.deserialize_uint8()?);
                }
                _ => (),
            }
            Ok(())
        }

        fn visit_array<E: DeserError, AD: deserialize::ArrayDeserializer<Error = E>>(
            &mut self,
            field_name: &str,
            mut array_deserializer: AD,
        ) -> Result<(), E> {
            #[allow(clippy::single_match)]
            match field_name {
                "Memos" => {
                    while let Some(memo) = array_deserializer.deserialize_object("Memo")? {
                        self.objects.push(memo);
                    }
                }
                _ => {}
            }
            Ok(())
        }
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

    #[test]
    fn test_read_field_id_4bit_type_4bit_field() {
        let mut s = deserializer(&[0b0010_0100]);
        let value = s.read_field_id().unwrap();
        assert_eq!(
            value,
            FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0100))
        );
    }

    #[test]
    fn test_read_field_id_4bit_type_8bit_field() {
        let mut s = deserializer(&[0b0010_0000, 0b0001_0100]);
        let value = s.read_field_id().unwrap();
        assert_eq!(
            value,
            FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0001_0100))
        );
    }

    #[test]
    fn test_read_field_id_8bit_type_8bit_field() {
        let mut s = deserializer(&[0, 0b0001_0001, 0b0001_0100]);
        let value = s.read_field_id().unwrap();
        assert_eq!(
            value,
            FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0001_0100))
        );
    }

    #[test]
    fn test_read_field_id_8bit_type_4bit_field() {
        let mut s = deserializer(&[0b0000_0100, 0b0001_0001]);
        let value = s.read_field_id().unwrap();
        assert_eq!(
            value,
            FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0100))
        );
    }

    /// Test reading array of objects
    #[test]
    fn test_read_empty_array() {
        let s = deserializer(&[0b1111_1001, 0b1111_0001]);

        let mut visitor = ObjectWithArrayVisitor::default();
        s.deserialize(&mut visitor).unwrap();
        assert_eq!(visitor.objects.len(), 0);
    }

    /// Test read array of objects
    #[test]
    fn test_read_array() {
        let s = deserializer(&[
            0b1111_1001,
            0b1110_1010,
            0b0010_0001,
            0,
            0,
            0,
            12,
            0b0010_0010,
            0,
            0,
            0,
            23,
            0b1110_0001,
            0b1110_1010,
            0b0010_0001,
            0,
            0,
            0,
            34,
            0b0010_0010,
            0,
            0,
            0,
            45,
            0b1110_0001,
            0b1111_0001,
        ]);

        let mut visitor = ObjectWithArrayVisitor::default();
        s.deserialize(&mut visitor).unwrap();
        assert_eq!(visitor.objects.len(), 2);
        assert_eq!(visitor.objects[0].field1, 12);
        assert_eq!(visitor.objects[0].field2, 23);
        assert_eq!(visitor.objects[1].field1, 34);
        assert_eq!(visitor.objects[1].field2, 45);
    }

    #[test]
    fn test_read_array_with_field_after() {
        let s = deserializer(&[
            0b1111_1001,
            0b1110_1010,
            0b0010_0001,
            0,
            0,
            0,
            12,
            0b0010_0010,
            0,
            0,
            0,
            23,
            0b1110_0001,
            0b1111_0001,
            0b0000_0000,
            0b0001_0000,
            0b0001_0000,
            12,
        ]);

        let mut visitor = ObjectWithArrayVisitor::default();
        s.deserialize(&mut visitor).unwrap();
        assert_eq!(visitor.objects.len(), 1);
        assert_eq!(visitor.tick_size, Some(12));
    }

    /// Test deserialize fields one by one (in order)
    #[test]
    fn test_deserialize_fields() {
        let mut s = deserializer(&[
            0b0010_0001,
            0,
            0,
            0,
            12,
            0b0010_0010,
            0,
            0,
            0,
            23,
            0b0011_0001,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            34,
        ]);

        assert_eq!(
            s.deserialize_single_field("NetworkID")
                .unwrap()
                .deserialize_uint32()
                .unwrap(),
            12
        );
        assert_eq!(
            s.deserialize_single_field("Flags")
                .unwrap()
                .deserialize_uint32()
                .unwrap(),
            23
        );
        assert_eq!(
            s.deserialize_single_field("IndexNext")
                .unwrap()
                .deserialize_uint64()
                .unwrap(),
            34
        );
    }

    /// Don't accept fields in wrong order (could make interpretation of transaction ambiguous
    /// depending on how other systems/implementations deserialize). For security reasons, follow the specification
    /// strictly.
    #[test]
    fn test_deserialize_fields_wrong_order() {
        let mut s = deserializer(&[0b0010_0010, 0, 0, 0, 23, 0b0010_0001, 0, 0, 0, 12]);

        s.deserialize_single_field("Flags")
            .unwrap()
            .deserialize_uint32()
            .unwrap();

        let result = s.deserialize_single_field("NetworkID");

        assert_matches!(result.map(|_|()), Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Field out of order"), "message: {}", message);
        });
    }

    /// Don't accept duplicate fields (could make interpretation of transaction ambiguous
    /// depending on how other systems/implementations deserialize). For security reasons, follow the specification
    /// strictly.
    #[test]
    fn test_deserialize_field_appears_twice() {
        let mut s = deserializer(&[0b0010_0010, 0, 0, 0, 23, 0b0010_0010, 0, 0, 0, 23]);

        s.deserialize_single_field("Flags")
            .unwrap()
            .deserialize_uint32()
            .unwrap();

        let result = s.deserialize_single_field("Flags");

        assert_matches!(result.map(|_|()), Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Field appears twice"), "message: {}", message);
        });
    }

    #[test]
    fn test_deserialize_field_wrong_name() {
        let mut s = deserializer(&[
            0b0010_0001, // NetworkID field
            0,
            0,
            0,
            12,
        ]);

        let result = s.deserialize_single_field("Flags");

        assert_matches!(result.map(|_|()), Err(BinaryCodecError::InvalidField(message)) => {
            assert!(message.contains("Expected field"), "message: {}", message);
        });
    }

    #[test]
    fn test_deserialize_field_wrong_type() {
        let mut s = deserializer(&[
            0b0010_0001, // type uint32
            0,
            0,
            0,
            12,
        ]);

        let result = s
            .deserialize_single_field("NetworkID")
            .unwrap()
            .deserialize_uint64();

        assert_matches!(result.map(|_|()), Err(BinaryCodecError::InvalidField(message)) => {
            assert!(message.contains("Expected type"), "message: {}", message);
        });
    }

    /// Test deserialize with visitor
    #[test]
    fn test_deserialize_fields_visitor() {
        let s = deserializer(&[0b0010_0001, 0, 0, 0, 12, 0b0010_0010, 0, 0, 0, 23]);

        #[derive(Default)]
        struct Visitor {
            network_id: Option<UInt32>,
            flags: Option<UInt32>,
        }

        impl deserialize::Visitor for Visitor {
            fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                &mut self,
                field_name: &str,
                field_accessor: F,
            ) -> Result<(), E> {
                match field_name {
                    "NetworkID" => {
                        self.network_id = Some(field_accessor.deserialize_uint32()?);
                    }
                    "Flags" => {
                        self.flags = Some(field_accessor.deserialize_uint32()?);
                    }
                    _ => (),
                }
                Ok(())
            }

            fn visit_array<E: DeserError, AD: deserialize::ArrayDeserializer<Error = E>>(
                &mut self,
                _field_name: &str,
                _array_deserializer: AD,
            ) -> Result<(), E> {
                Ok(())
            }
        }

        let mut visitor = Visitor::default();

        s.deserialize(&mut visitor).unwrap();

        assert_eq!(visitor.network_id, Some(12));
        assert_eq!(visitor.flags, Some(23));
    }

    /// Test deserialize with visitor, where a field is not used.
    #[test]
    fn test_deserialize_fields_unused_field() {
        // first field is NetworkID which we ignore
        let s = deserializer(&[0b0010_0001, 0, 0, 0, 12, 0b0010_0010, 0, 0, 0, 23]);

        #[derive(Default)]
        struct Visitor {
            flags: Option<UInt32>,
        }

        impl deserialize::Visitor for Visitor {
            fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                &mut self,
                field_name: &str,
                field_accessor: F,
            ) -> Result<(), E> {
                match field_name {
                    "Flags" => {
                        self.flags = Some(field_accessor.deserialize_uint32()?);
                    }
                    _ => return Err(E::unexpected_field(field_name)),
                }
                Ok(())
            }

            fn visit_array<E: DeserError, AD: deserialize::ArrayDeserializer<Error = E>>(
                &mut self,
                field_name: &str,
                _array_deserializer: AD,
            ) -> Result<(), E> {
                Err(E::unexpected_field(field_name))
            }
        }

        let mut visitor = Visitor::default();

        let result = s.deserialize(&mut visitor);

        assert_matches!(result.map(|_|()), Err(BinaryCodecError::InvalidField(message)) => {
            assert!(message.contains("Unexpected field"), "message: {}", message);
        });
    }

    /// Tests the example <https://xrpl.org/serialization.html#examples>
    #[test]
    fn test_deserialize_offer_create() {
        let txn: OfferCreateTransaction = crate::deserialize::deserialize(&hex::decode("120007220008000024001ABED82A2380BF2C2019001ABED764D55920AC9391400000000000000000000000000055534400000000000A20B3C85F482532A9578DBB3950B85CA06594D165400000037E11D60068400000000000000A732103EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3744630440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C8114DD76483FACDEE26E60D8A586BB58D09F27045C46").unwrap()).unwrap();

        assert_eq!(
            txn.common.account,
            AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap()
        );
        assert_eq!(txn.taker_gets, Amount::drops(15000000000).unwrap());
        assert_eq!(
            txn.taker_pays,
            Amount::issued(
                IssuedValue::from_mantissa_exponent(70728, -1).unwrap(),
                CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
                AccountId::from_address("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B").unwrap(),
            )
            .unwrap()
        );
        assert_eq!(txn.common.fee, Some(DropsAmount::from_drops(10).unwrap()));
        assert_eq!(txn.common.sequence, Some(1752792));
        assert_eq!(
            txn.common.signing_pub_key,
            Some(Blob(
                hex::decode("03EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3")
                    .unwrap(),
            ))
        );
        assert_eq!(txn.common.txn_signature, Some(Blob(hex::decode("30440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C").unwrap())));
        assert_eq!(txn.expiration, Some(595640108));
        assert_eq!(txn.flags, BitFlags::from_bits(524288).unwrap());
        assert_eq!(txn.offer_sequence, Some(1752791));
    }

    /// Deserialize transaction with `Memos` array field
    #[test]
    fn test_deserialize_transaction_with_memos() {
        let mut txn_orig = AccountSetTransaction::new(
            AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap(),
        );
        txn_orig.common.memos.push(Memo {
            memo_type: Blob(vec![0, 1]),
            memo_data: Blob(vec![2, 3]),
            memo_format: None,
        });
        txn_orig.common.memos.push(Memo {
            memo_type: Blob(vec![4, 5]),
            memo_data: Blob(vec![6, 7]),
            memo_format: None,
        });

        let txn: AccountSetTransaction =
            crate::deserialize::deserialize(&serialize::serialize(&txn_orig).unwrap()).unwrap();
        assert_eq!(txn.common.memos.len(), 2);
        assert_eq!(
            txn.common.account,
            AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap()
        );
    }

    /// Deserialize to `Transaction` enum type
    #[test]
    fn test_deserialize_as_transaction() {
        let txn_orig = AccountSetTransaction::new(
            AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap(),
        );
        let txn: Transaction =
            crate::deserialize::deserialize(&serialize::serialize(&txn_orig).unwrap()).unwrap();
        assert_matches!(txn, Transaction::AccountSet(txn) => {
            assert_eq!(txn.common.account, AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap());
        });
    }
}
