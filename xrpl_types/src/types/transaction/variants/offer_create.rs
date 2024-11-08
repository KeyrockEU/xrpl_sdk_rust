use crate::deserialize::{ArrayDeserializer, DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, Serializer};
use crate::{
    deserialize, AccountId, Amount, TransactionCommon, TransactionCommonVisitor, TransactionTrait,
    TransactionType, UInt32,
};
use enumflags2::{bitflags, BitFlags};

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone)]
pub struct OfferCreateTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<OfferCreateFlags>,
    pub expiration: Option<UInt32>,
    pub offer_sequence: Option<UInt32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

impl OfferCreateTransaction {
    pub fn new(account_id: AccountId, taker_gets: Amount, taker_pays: Amount) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            expiration: None,
            offer_sequence: None,
            taker_gets,
            taker_pays,
        }
    }
}

impl TransactionTrait for OfferCreateTransaction {
    fn common(&self) -> &TransactionCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut TransactionCommon {
        &mut self.common
    }
}

/// `OfferCreate` flags <https://xrpl.org/offercreate.html#offercreate-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferCreateFlags {
    FullyCanonicalSig = 0x80000000,
    Passive = 0x00010000,
    ImmediateOrCancel = 0x00020000,
    FillOrKill = 0x00040000,
    Sell = 0x00080000,
}

impl Serialize for OfferCreateTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::OfferCreate as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        if let Some(expiration) = self.expiration {
            s.serialize_uint32("Expiration", expiration)?;
        }
        if let Some(offer_sequence) = self.offer_sequence {
            s.serialize_uint32("OfferSequence", offer_sequence)?;
        }
        s.serialize_amount("TakerPays", self.taker_pays)?;
        s.serialize_amount("TakerGets", self.taker_gets)?;
        Ok(())
    }
}

impl Deserialize for OfferCreateTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<OfferCreateFlags>,
            expiration: Option<UInt32>,
            offer_sequence: Option<UInt32>,
            taker_gets: Option<Amount>,
            taker_pays: Option<Amount>,
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
                            != TransactionType::OfferCreate as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "Expiration" => {
                        self.expiration = Some(field_accessor.deserialize_uint32()?);
                    }
                    "OfferSequence" => {
                        self.offer_sequence = Some(field_accessor.deserialize_uint32()?);
                    }
                    "TakerPays" => {
                        self.taker_pays = Some(field_accessor.deserialize_amount()?);
                    }
                    "TakerGets" => {
                        self.taker_gets = Some(field_accessor.deserialize_amount()?);
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

        Ok(OfferCreateTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            expiration: visitor.expiration,
            offer_sequence: visitor.offer_sequence,
            taker_gets: S::Error::unwrap_field_value("TakerGets", visitor.taker_gets)?,
            taker_pays: S::Error::unwrap_field_value("TakerPays", visitor.taker_pays)?,
        })
    }
}
