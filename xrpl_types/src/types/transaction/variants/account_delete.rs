use crate::deserialize::{DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, Serializer};
use crate::{
    deserialize, AccountId, TransactionCommon, TransactionCommonVisitor, TransactionTrait,
    TransactionType, UInt32,
};
use enumflags2::{bitflags, BitFlags};

/// An `AccountDelete` transaction <https://xrpl.org/accountdelete.html>
#[derive(Debug, Clone)]
pub struct AccountDeleteTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<AccountDeleteFlags>,
    pub destination: AccountId,
    pub destination_tag: Option<UInt32>,
}

impl AccountDeleteTransaction {
    pub fn new(account_id: AccountId, destination: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            destination,
            destination_tag: None,
        }
    }
}

impl TransactionTrait for AccountDeleteTransaction {
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
pub enum AccountDeleteFlags {
    FullyCanonicalSig = 0x80000000,
}

impl Serialize for AccountDeleteTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::AccountDelete as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_account_id("Destination", self.destination)?;
        if let Some(destination_tag) = self.destination_tag {
            s.serialize_uint32("DestinationTag", destination_tag)?;
        }
        Ok(())
    }
}

impl Deserialize for AccountDeleteTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<AccountDeleteFlags>,
            destination: Option<AccountId>,
            destination_tag: Option<UInt32>,
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
                            != TransactionType::AccountDelete as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "Destination" => {
                        self.destination = Some(field_accessor.deserialize_account_id()?);
                    }
                    "DestinationTag" => {
                        self.destination_tag = Some(field_accessor.deserialize_uint32()?);
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

        Ok(AccountDeleteTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            destination: S::Error::unwrap_field_value("Destination", visitor.destination)?,
            destination_tag: visitor.destination_tag,
        })
    }
}
