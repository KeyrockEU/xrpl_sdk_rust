use crate::deserialize::{ArrayDeserializer, DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, Serializer};
use crate::{
    deserialize, AccountId, TransactionCommon, TransactionCommonVisitor, TransactionTrait,
    TransactionType, UInt32,
};
use enumflags2::{bitflags, BitFlags};

/// An `OfferCancel` transaction <https://xrpl.org/offercancel.html>
#[derive(Debug, Clone)]
pub struct OfferCancelTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<OfferCancelFlags>,
    pub offer_sequence: UInt32,
}

impl OfferCancelTransaction {
    pub fn new(account_id: AccountId, offer_sequence: UInt32) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            offer_sequence,
        }
    }
}

impl TransactionTrait for OfferCancelTransaction {
    fn common(&self) -> &TransactionCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut TransactionCommon {
        &mut self.common
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferCancelFlags {
    FullyCanonicalSig = 0x80000000,
}

impl Serialize for OfferCancelTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::OfferCancel as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_uint32("OfferSequence", self.offer_sequence)?;
        Ok(())
    }
}

impl Deserialize for OfferCancelTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<OfferCancelFlags>,
            offer_sequence: Option<UInt32>,
        }

        impl deserialize::Visitor for Visitor {
            fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                &mut self,
                field_name: &str,
                field_accessor: F,
            ) -> Result<(), E> {
                match field_name {
                    "TransactionType" => {
                        if field_accessor.deserialize_uint16()?
                            != TransactionType::OfferCancel as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "OfferSequence" => {
                        self.offer_sequence = Some(field_accessor.deserialize_uint32()?);
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

        Ok(OfferCancelTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            offer_sequence: S::Error::unwrap_field_value("OfferSequence", visitor.offer_sequence)?,
        })
    }
}
