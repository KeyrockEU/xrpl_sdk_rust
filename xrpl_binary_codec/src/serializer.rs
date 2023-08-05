use crate::error::BinaryCodecError;
use std::io::Write;
use xrpl_types::serialize::{FieldCode, FieldId, TypeCode};
use xrpl_types::Uint64;
use xrpl_types::{
    AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256, IssuedValue,
    UInt16, UInt32, UInt8,
};

// todo allan
pub const HASH_PREFIX_TRANSACTION: [u8; 4] = [0x53, 0x4E, 0x44, 0x00];
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

pub struct Serializer<W> {
    writer: W,
    /// Previously serialized field id
    prev_field_id: Option<FieldId>,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            prev_field_id: None,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write> xrpl_types::serialize::Serializer for Serializer<W> {
    type Error = BinaryCodecError;

    fn serialize_account_id(
        &mut self,
        field_code: FieldCode,
        account_id: AccountId,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::AccountId, field_code))?;
        self.push_account_id(account_id)?;
        Ok(())
    }

    fn serialize_amount(
        &mut self,
        field_code: FieldCode,
        amount: Amount,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::Amount, field_code))?;
        self.push_amount(amount)?;
        Ok(())
    }

    fn serialize_blob(
        &mut self,
        field_code: FieldCode,
        blob: &Blob,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::Blob, field_code))?;
        self.push_blob(blob)?;
        Ok(())
    }

    fn serialize_hash128(
        &mut self,
        field_code: FieldCode,
        hash128: Hash128,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::Hash128, field_code))?;
        self.push_hash128(hash128)?;
        Ok(())
    }

    fn serialize_hash160(
        &mut self,
        field_code: FieldCode,
        hash160: Hash160,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::Hash160, field_code))?;
        self.push_hash160(hash160)?;
        Ok(())
    }

    fn serialize_hash256(
        &mut self,
        field_code: FieldCode,
        hash256: Hash256,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::Hash256, field_code))?;
        self.push_hash256(hash256)?;
        Ok(())
    }

    fn serialize_uint8(
        &mut self,
        field_code: FieldCode,
        uint8: UInt8,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::UInt8, field_code))?;
        self.push_uint8(uint8)?;
        Ok(())
    }

    fn serialize_uint16(
        &mut self,
        field_code: FieldCode,
        uint16: UInt16,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::UInt16, field_code))?;
        self.push_uint16(uint16)?;
        Ok(())
    }

    fn serialize_uint32(
        &mut self,
        field_code: FieldCode,
        uint32: UInt32,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::UInt32, field_code))?;
        self.push_uint32(uint32)?;
        Ok(())
    }

    fn serialize_uint64(
        &mut self,
        field_code: FieldCode,
        uint64: Uint64,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(FieldId::from_type_field(TypeCode::UInt64, field_code))?;
        self.push_uint64(uint64)?;
        Ok(())
    }
}

impl<W: Write> Serializer<W> {
    fn push(&mut self, value: u8) -> Result<(), BinaryCodecError> {
        self.push_slice(&[value])
    }

    fn push_slice(&mut self, bytes: &[u8]) -> Result<(), BinaryCodecError> {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn push_uint8(&mut self, value: UInt8) -> Result<(), BinaryCodecError> {
        self.push(value)
    }

    fn push_uint16(&mut self, value: UInt16) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_uint32(&mut self, value: UInt32) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_uint64(&mut self, value: Uint64) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_hash128(&mut self, value: Hash128) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_hash160(&mut self, value: Hash160) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_hash256(&mut self, value: Hash256) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_blob(&mut self, blob: &Blob) -> Result<(), BinaryCodecError> {
        self.push_vl_prefix(blob.0.len())?;
        self.push_slice(&blob.0)?;
        Ok(())
    }

    /// Push field id <https://xrpl.org/serialization.html#field-ids>
    fn push_field_id(&mut self, field_id: FieldId) -> Result<(), BinaryCodecError> {
        // rippled implementation: https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148

        let type_code = field_id.type_code as u8;
        let field_code = field_id.field_code.0;

        if let Some(prev_field_id) = self.prev_field_id {
            if field_id <= prev_field_id {
                return Err(BinaryCodecError::FieldOrder(
                    "Order of serialized fields is wrong".to_string(),
                ));
            }
        }

        self.prev_field_id = Some(field_id);

        if type_code < 16 && field_code < 16 {
            self.push(type_code << 4 | field_code)?;
        } else if type_code < 16 {
            self.push(type_code << 4)?;
            self.push(field_code)?;
        } else if field_code < 16 {
            self.push(field_code)?;
            self.push(type_code)?;
        } else {
            self.push(0)?;
            self.push(type_code)?;
            self.push(field_code)?;
        }
        Ok(())
    }

    /// Push length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    fn push_vl_prefix(&mut self, length: usize) -> Result<(), BinaryCodecError> {
        if length <= 192 {
            self.push(length as u8)?;
            Ok(())
        } else if length <= 12480 {
            let length = length - 193;
            self.push(193 + (length >> 8) as u8)?;
            self.push((length & 0xff) as u8)?;
            Ok(())
        } else if length <= 918744 {
            let length = length - 12481;
            self.push(241 + (length >> 16) as u8)?;
            self.push(((length >> 8) & 0xff) as u8)?;
            self.push((length & 0xff) as u8)?;
            Ok(())
        } else {
            Err(BinaryCodecError::OutOfRange(format!(
                "Variable length out of range: {}",
                length
            )))
        }
    }

    /// <https://xrpl.org/serialization.html#amount-fields>
    fn push_drops_amount(&mut self, drops: DropsAmount) -> Result<(), BinaryCodecError> {
        const POSITIVE_MASK: u64 = 0x4000000000000000;
        self.push_uint64(POSITIVE_MASK | drops.drops())
    }

    /// <https://xrpl.org/serialization.html#issued-currency-amount-format>
    fn push_issued_value(&mut self, value: IssuedValue) -> Result<(), BinaryCodecError> {
        const ISSUED_MASK: u64 = 0x8000000000000000;
        const POSITIVE_MASK: u64 = 0x4000000000000000;

        let (mantissa, positive) = match value.mantissa() {
            0 => {
                self.push_uint64(ISSUED_MASK)?;
                return Ok(());
            }
            1.. => (value.mantissa() as u64, true),
            ..=-1 => (-value.mantissa() as u64, false),
        };
        let exponent = (value.exponent() + 97) as u64;
        self.push_uint64(
            ISSUED_MASK | (if positive { POSITIVE_MASK } else { 0 }) | mantissa | (exponent << 54),
        )?;
        Ok(())
    }

    fn push_amount(&mut self, amount: Amount) -> Result<(), BinaryCodecError> {
        match amount {
            Amount::Drops(drops) => self.push_drops_amount(drops),
            Amount::Issued(issued) => {
                self.push_issued_value(issued.value())?;
                self.push_currency_code(issued.currency())?;
                self.push_account_id_no_length_prefix(issued.issuer())?;
                Ok(())
            }
        }
    }

    /// <https://xrpl.org/serialization.html#currency-codes>
    fn push_currency_code(&mut self, currency_code: CurrencyCode) -> Result<(), BinaryCodecError> {
        match currency_code {
            CurrencyCode::Xrp => self.push_slice(&[0u8; 20]),
            CurrencyCode::Standard(code) => {
                self.push_slice(&[0u8; 12])?;
                self.push_slice(&code.as_bytes())?;
                self.push_slice(&[0u8; 5])?;
                Ok(())
            }
            CurrencyCode::NonStandard(code) => self.push_slice(code.as_bytes()),
        }
    }

    fn push_account_id(&mut self, id: AccountId) -> Result<(), BinaryCodecError> {
        self.push_vl_prefix(20).expect("20 is within valid range");
        self.push_slice(&id.0)
    }

    fn push_account_id_no_length_prefix(&mut self, id: AccountId) -> Result<(), BinaryCodecError> {
        self.push_slice(&id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use enumflags2::BitFlags;
    use xrpl_types::serialize::{FieldCode, Serialize, Serializer};
    use xrpl_types::{OfferCreateTransaction, Transaction, TransactionType};

    fn serializer() -> super::Serializer<Vec<u8>> {
        super::Serializer::new(Vec::new())
    }

    #[test]
    fn test_push_uint8() {
        let mut s = serializer();
        let value = 0x12;
        s.push_uint8(value).unwrap();
        assert_eq!(s.into_inner(), [0x12u8]);
    }

    #[test]
    fn test_push_uint16() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 8);
        s.push_uint16(value).unwrap();
        assert_eq!(s.into_inner(), [0x34, 0x12]);
    }

    #[test]
    fn test_push_uint32() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 24);
        s.push_uint32(value).unwrap();
        assert_eq!(s.into_inner(), [0x34, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn test_push_uint64() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 56);
        s.push_uint64(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12]
        );
    }

    #[test]
    fn test_push_h128() {
        let mut s = serializer();
        let value = Hash128([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x12,
        ]);
        s.push_hash128(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h160() {
        let mut s = serializer();
        let value = Hash160([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash160(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h256() {
        let mut s = serializer();
        let value = Hash256([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash256(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_blob() {
        let mut s = serializer();
        let value = Blob(vec![0x34, 0x00, 0x12]);
        s.push_blob(&value).unwrap();
        assert_eq!(s.into_inner(), [3, 0x34, 0x00, 0x12]);
    }

    #[test]
    fn test_push_account_id() {
        let mut s = serializer();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                20, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_account_id_no_length_prefix() {
        let mut s = serializer();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id_no_length_prefix(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    /// Tests length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    #[test]
    fn test_push_vl_prefix() {
        // test range 0 to 192
        let mut s = serializer();
        s.push_vl_prefix(0).unwrap();
        s.push_vl_prefix(1).unwrap();
        s.push_vl_prefix(192).unwrap();
        assert_eq!(s.into_inner(), [0, 1, 192]);

        // test range 193 to 12480
        let mut s = serializer();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 0).unwrap();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 1).unwrap();
        assert_eq!(193 + ((240 - 193) * 256) + 255, 12480);
        s.push_vl_prefix(193 + ((240 - 193) * 256) + 255).unwrap();
        assert_eq!(s.into_inner(), [193, 0, 193, 1, 240, 255]);

        // test range 12481 to 918744
        let mut s = serializer();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (0 * 256) + 0)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (0 * 256) + 1)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (1 * 256) + 0)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (255 * 256) + 255)
            .unwrap();
        assert_eq!(12481 + ((254 - 241) * 65536) + (212 * 256) + 23, 918744);
        s.push_vl_prefix(12481 + ((254 - 241) * 65536) + (212 * 256) + 23)
            .unwrap();
        assert_eq!(
            s.into_inner(),
            [241, 0, 0, 241, 0, 1, 241, 1, 0, 241, 255, 255, 254, 212, 23]
        );

        // test out of range
        let mut s = serializer();
        let result = s.push_vl_prefix(918745);
        assert_matches!(result, Err(BinaryCodecError::OutOfRange(message)) => {
            assert!(message.contains("Variable length out of range"), "message: {}", message);
        });
    }

    #[test]
    fn test_push_currency_code_xrp() {
        let mut s = serializer();
        let code = CurrencyCode::xrp();
        s.push_currency_code(code).unwrap();
        assert_eq!(s.into_inner(), [0u8; 20]);
    }

    #[test]
    fn test_push_currency_code_standard() {
        let mut s = serializer();
        let code = CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap();
        s.push_currency_code(code).unwrap();
        let bytes = s.into_inner();
        assert_eq!(bytes[0..12], [0u8; 12]);
        assert_eq!(
            bytes[12..15],
            [
                AsciiChar::U.as_byte(),
                AsciiChar::S.as_byte(),
                AsciiChar::D.as_byte()
            ]
        );
        assert_eq!(bytes[15..20], [0u8; 5]);
    }

    #[test]
    fn test_push_currency_code_non_standard() {
        let mut s = serializer();
        let code = CurrencyCode::non_standard([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        s.push_currency_code(code).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ]
        );
    }

    #[test]
    fn test_push_drops_amount() {
        let mut s = serializer();
        let value = DropsAmount::from_drops(10_000).unwrap();
        s.push_drops_amount(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]
        );
    }

    /// Test serializing zero issued value
    #[test]
    fn test_push_issued_value_zero() {
        let mut s = serializer();
        let value = IssuedValue::zero();
        s.push_issued_value(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    /// Test serializing positive issued value
    #[test]
    fn test_push_issued_value_positive() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    /// Test serializing negative issued value
    #[test]
    fn test_push_issued_value_negative() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(-1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [0x95, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    #[test]
    fn test_push_amount_drops() {
        let mut s = serializer();
        let value = Amount::drops(10_000).unwrap();
        s.push_amount(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]
        );
    }

    #[test]
    fn test_push_amount_issued() {
        let mut s = serializer();
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
        let amount = Amount::issued(value, currency, issuer).unwrap();
        s.push_amount(amount).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [
                0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    #[test]
    fn test_push_field_id_4bit_type_4bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0010_0100]);
    }

    #[test]
    fn test_push_field_id_4bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0010_0000, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0, 0b0001_0001, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_4bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0000_0100, 0b0001_0001]);
    }

    /// Test serialize fields (in correct order)
    #[test]
    fn test_serialize_fields() {
        let mut s = serializer();
        s.serialize_uint32(FieldCode(1), 12).unwrap();
        s.serialize_uint32(FieldCode(2), 23).unwrap();
        s.serialize_uint64(FieldCode(1), 34).unwrap();
        assert_eq!(
            s.into_inner(),
            [
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
                34
            ]
        );
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_wrong_type_code_order() {
        let mut s = serializer();
        s.serialize_uint64(FieldCode(1), 34).unwrap();
        let result = s.serialize_uint32(FieldCode(2), 12);
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Order of serialized fields is wrong"), "message: {}", message);
        });
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_wrong_field_code_order() {
        let mut s = serializer();
        s.serialize_uint32(FieldCode(2), 12).unwrap();
        let result = s.serialize_uint32(FieldCode(1), 34);
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Order of serialized fields is wrong"), "message: {}", message);
        });
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_same_field_id() {
        let mut s = serializer();
        s.serialize_uint32(FieldCode(2), 34).unwrap();
        let result = s.serialize_uint32(FieldCode(2), 12);
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Order of serialized fields is wrong"), "message: {}", message);
        });
    }

    /// Tests the example <https://xrpl.org/serialization.html#examples>
    #[test]
    fn test_serialize_offer_create() {
        let mut s = serializer();
        let tx = OfferCreateTransaction {
            common: Transaction {
                account: AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap(),
                transaction_type: TransactionType::OfferCreate,
                fee: Some(DropsAmount::from_drops(10).unwrap()),
                sequence: Some(1752792),
                account_txn_id: None,
                flags: BitFlags::default(),
                last_ledger_sequence: None,
                network_id: None,
                source_tag: None,
                signing_pub_key: Some(Blob(hex::decode("03EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3").unwrap())),
                ticket_sequence: None,
                txn_signature: Some(Blob(hex::decode("30440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C").unwrap())),
            },
            expiration: Some(595640108),
            flags: BitFlags::from_bits(524288).unwrap(),
            offer_sequence: Some(1752791),
            taker_gets: Amount::drops(15000000000).unwrap(),
            taker_pays: Amount::issued(
                IssuedValue::from_mantissa_exponent(70728, -1).unwrap(),
                CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
                AccountId::from_address("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B").unwrap(),
            ).unwrap(),
        };

        tx.serialize(&mut s).unwrap();
        assert_eq!(hex::encode_upper(s.into_inner()), "120007220008000024001ABED82A2380BF2C2019001ABED764D55920AC9391400000000000000000000000000055534400000000000A20B3C85F482532A9578DBB3950B85CA06594D165400000037E11D60068400000000000000A732103EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3744630440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C8114DD76483FACDEE26E60D8A586BB58D09F27045C46");
    }
}
