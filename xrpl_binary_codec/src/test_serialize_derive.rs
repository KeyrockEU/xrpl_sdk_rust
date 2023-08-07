use enumflags2::BitFlags;
use xrpl_types::serialize::Serialize;
use xrpl_types::{IssuedAmount, TransactionCommon, TrustSetFlags, UInt32};

// todo allan remove
/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Serialize)]
pub struct TrustSetTransaction {
    #[xrpl_binary(flatten)]
    pub common: TransactionCommon,
    pub flags: BitFlags<TrustSetFlags>,
    pub limit_amount: IssuedAmount,
    pub quality_in: Option<UInt32>,
    pub quality_out: Option<UInt32>,
}