use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, Amount, Hash256, TransactionTrait, TransactionCommon, TransactionType, UInt32, TransactionCommonVisitor, deserialize};
use enumflags2::{bitflags, BitFlags};
use crate::deserialize::{DeserError, Deserialize, Deserializer, FieldAccessor};

/// An `Payment` transaction <https://xrpl.org/payment.html>
#[derive(Debug, Clone)]
pub struct PaymentTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<PaymentFlags>,
    pub amount: Amount,
    pub destination: AccountId,
    pub destination_tag: Option<UInt32>,
    pub invoice_id: Option<Hash256>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

impl PaymentTransaction {
    pub fn new(account_id: AccountId, amount: Amount, destination: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            amount,
            destination,
            destination_tag: None,
            invoice_id: None,
            send_max: None,
            deliver_min: None,
        }
    }
}

impl TransactionTrait for PaymentTransaction {
    fn common(&self) -> &TransactionCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut TransactionCommon {
        &mut self.common
    }
}

/// `Payment` flags <https://xrpl.org/payment.html#payment-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlags {
    FullyCanonicalSig = 0x80000000,
    NoDirectRipple = 0x00010000,
    PartialPayment = 0x00020000,
    LimitQuality = 0x00040000,
}

impl Serialize for PaymentTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::Payment as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_amount("Amount", self.amount)?;
        s.serialize_account_id("Destination", self.destination)?;
        if let Some(destination_tag) = self.destination_tag {
            s.serialize_uint32("DestinationTag", destination_tag)?;
        }
        if let Some(invoice_id) = self.invoice_id {
            s.serialize_hash256("InvoiceID", invoice_id)?;
        }
        if let Some(send_max) = self.send_max {
            s.serialize_amount("SendMax", send_max)?;
        }
        if let Some(deliver_min) = self.deliver_min {
            s.serialize_amount("DeliverMin", deliver_min)?;
        }
        Ok(())
    }
}


impl Deserialize for PaymentTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<PaymentFlags>,
            amount: Option<Amount>,
            destination: Option<AccountId>,
            destination_tag: Option<UInt32>,
            invoice_id: Option<Hash256>,
            send_max: Option<Amount>,
            deliver_min: Option<Amount>,
        }

        impl deserialize::Visitor for Visitor {
            fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
                &mut self,
                field_name: &str,
                mut field_accessor: F,
            ) -> Result<(), E> {
                match field_name {
                    "TransactionType" => {
                        if field_accessor.deserialize_uint16()?
                            != TransactionType::Payment as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "Amount" => {
                        self.amount = Some(field_accessor.deserialize_amount()?);
                    }
                    "Destination" => {
                        self.destination = Some(field_accessor.deserialize_account_id()?);
                    }
                    "DestinationTag" => {
                        self.destination_tag = Some(field_accessor.deserialize_uint32()?);
                    }
                    "InvoiceID" => {
                        self.invoice_id = Some(field_accessor.deserialize_hash256()?);
                    }
                    "SendMax" => {
                        self.send_max = Some(field_accessor.deserialize_amount()?);
                    }
                    "DeliverMin" => {
                        self.deliver_min = Some(field_accessor.deserialize_amount()?);
                    }
                    _ => {
                        self.common.visit_field(field_name, field_accessor)?;
                    }
                }
                Ok(())
            }
        }

        let mut visitor = Visitor::default();

        deserializer.deserialize(&mut visitor)?;

        Ok(PaymentTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            amount: S::Error::unwrap_field_value("Amount", visitor.amount)?,
            destination: S::Error::unwrap_field_value("Destination", visitor.destination)?,
            destination_tag: visitor.destination_tag,
            invoice_id: visitor.invoice_id,
            send_max: visitor.send_max,
            deliver_min: visitor.deliver_min,
        })
    }
}

