pub mod field_info;

use core::fmt;

/// Field data type codes <https://xrpl.org/serialization.html#type-list>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum TypeCode {
    // Discriminant values can be found at https://xrpl.org/serialization.html#type-list and also at https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json
    AccountId = 8,
    Amount = 6,
    Blob = 7,
    Hash128 = 4,
    Hash160 = 17,
    Hash256 = 5,
    UInt8 = 16,
    UInt16 = 1,
    UInt32 = 2,
    UInt64 = 3,
    Array = 15,
    Object = 14,
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TypeCode {
    pub fn from_discriminant_opt(disc: u8) -> Option<Self> {
        match disc {
            8 => Some(Self::AccountId),
            6 => Some(Self::Amount),
            7 => Some(Self::Blob),
            4 => Some(Self::Hash128),
            17 => Some(Self::Hash160),
            5 => Some(Self::Hash256),
            16 => Some(Self::UInt8),
            1 => Some(Self::UInt16),
            2 => Some(Self::UInt32),
            3 => Some(Self::UInt64),
            15 => Some(Self::Array),
            14 => Some(Self::Object),
            _ => None,
        }
    }
}

/// Field code <https://xrpl.org/serialization.html#field-codes>. The code for a given field can be found at
/// <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json> or
/// <https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldCode(pub u8);

impl fmt::Display for FieldCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Ordered field id <https://xrpl.org/serialization.html#canonical-field-order>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FieldId {
    /// Type code <https://xrpl.org/serialization.html#type-codes>
    pub type_code: TypeCode,
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub field_code: FieldCode,
}

impl FieldId {
    pub fn from_type_field(type_code: TypeCode, field_code: FieldCode) -> Self {
        Self {
            type_code,
            field_code,
        }
    }
}
