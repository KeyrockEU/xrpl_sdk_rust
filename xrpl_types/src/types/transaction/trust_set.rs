use std::fmt::Debug;
use crate::serialize::{FieldCode, Serialize, Serializer};
use crate::{AccountId, Amount, Blob, IssuedAmount, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Debug, Clone)]
pub struct TrustSetTransaction {
    #[xrpl_binary(flatten)]
    pub common: TransactionCommon,
    pub flags: BitFlags<TrustSetFlags>,
    pub limit_amount: IssuedAmount,
    pub quality_in: Option<UInt32>,
    pub quality_out: Option<UInt32>,
}

impl TrustSetTransaction {
    pub fn new(account_id: AccountId, limit_amount: IssuedAmount) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            limit_amount,
            quality_in: None,
            quality_out: None,
        }
    }
}

/// `TrustSet` flags <https://xrpl.org/trustset.html#trustset-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrustSetFlags {
    FullyCanonicalSig = 0x80000000,
    SetfAuth = 0x00010000,
    SetNoRipple = 0x00020000,
    ClearNoRipple = 0x00040000,
    SetFreeze = 0x00100000,
    ClearFreeze = 0x00200000,
}


#[derive(Serialize)]
#[xrpl_binary(crate_path = "crate")]
pub struct TestObject {
    #[xrpl_binary(flatten)]
    pub common: TransactionCommon,
    pub limit_amount: Amount,
    pub quality_in: UInt32,
    pub quality_out: UInt32,
    pub txn_signature: Blob,

}