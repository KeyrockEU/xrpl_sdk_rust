use crate::deserialize::{ArrayDeserializer, DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, Serializer};
use crate::{
    deserialize, AccountId, Amount, IssuedAmount, TransactionCommon, TransactionCommonVisitor,
    TransactionTrait, TransactionType, UInt32,
};
use enumflags2::{bitflags, BitFlags};

/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Debug, Clone)]
pub struct TrustSetTransaction {
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

impl TransactionTrait for TrustSetTransaction {
    fn common(&self) -> &TransactionCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut TransactionCommon {
        &mut self.common
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

impl Serialize for TrustSetTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::TrustSet as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_amount("LimitAmount", Amount::Issued(self.limit_amount))?;
        if let Some(quality_in) = self.quality_in {
            s.serialize_uint32("QualityIn", quality_in)?;
        }
        if let Some(quality_out) = self.quality_out {
            s.serialize_uint32("QualityOut", quality_out)?;
        }
        Ok(())
    }
}

impl Deserialize for TrustSetTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<TrustSetFlags>,
            limit_amount: Option<IssuedAmount>,
            quality_in: Option<UInt32>,
            quality_out: Option<UInt32>,
        }

        impl deserialize::Visitor for Visitor {
            fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                &mut self,
                field_name: &str,
                field_accessor: F,
            ) -> Result<(), E> {
                match field_name {
                    "TransactionType" => {
                        if field_accessor.deserialize_uint16()? != TransactionType::TrustSet as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "LimitAmount" => {
                        self.limit_amount = Some(match field_accessor.deserialize_amount()? {
                            Amount::Drops(_) => {
                                return Err(E::invalid_value("Limit amount drops"));
                            }
                            Amount::Issued(amount) => amount,
                        });
                    }
                    "QualityIn" => {
                        self.quality_in = Some(field_accessor.deserialize_uint32()?);
                    }
                    "QualityOut" => {
                        self.quality_out = Some(field_accessor.deserialize_uint32()?);
                    }
                    _ => {
                        self.common.visit_field(field_name, field_accessor)?;
                    }
                }
                Ok(())
            }

            fn visit_array<E: DeserError, AD: ArrayDeserializer<Error = E>>(
                &mut self,
                field_name: &str,
                array_deserializer: AD,
            ) -> Result<(), E> {
                self.common.visit_array(field_name, array_deserializer)
            }
        }

        let mut visitor = Visitor::default();

        deserializer.deserialize(&mut visitor)?;

        Ok(TrustSetTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            limit_amount: S::Error::unwrap_field_value("LimitAmount", visitor.limit_amount)?,
            quality_in: visitor.quality_in,
            quality_out: visitor.quality_out,
        })
    }
}
