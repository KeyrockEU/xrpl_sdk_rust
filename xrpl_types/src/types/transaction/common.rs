use crate::alloc::vec::Vec;
use crate::deserialize::{DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, SerializeArray, Serializer};
use crate::{deserialize, AccountId, Amount, Blob, DropsAmount, Hash256, UInt32};

#[derive(Debug, Clone)]
pub struct Memo {
    pub memo_type: Blob,
    pub memo_data: Blob,
    pub memo_format: Option<Blob>,
}

/// A ledger transaction <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone, Default)]
pub struct TransactionCommon {
    pub account: AccountId,
    pub fee: Option<DropsAmount>,
    pub sequence: Option<UInt32>,
    pub account_txn_id: Option<Hash256>,
    pub last_ledger_sequence: Option<UInt32>,
    pub memos: Vec<Memo>,
    pub network_id: Option<UInt32>,
    pub source_tag: Option<UInt32>,
    pub signing_pub_key: Option<Blob>,
    pub ticket_sequence: Option<UInt32>,
    pub txn_signature: Option<Blob>,
}

impl TransactionCommon {
    pub fn new(account: AccountId) -> Self {
        Self {
            account,
            fee: None,
            sequence: None,
            account_txn_id: None,
            last_ledger_sequence: None,
            memos: Vec::default(),
            network_id: None,
            source_tag: None,
            signing_pub_key: None,
            ticket_sequence: None,
            txn_signature: None,
        }
    }
}

impl Serialize for TransactionCommon {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        if let Some(network_id) = self.network_id {
            s.serialize_uint32("NetworkID", network_id)?;
        }
        if let Some(source_tag) = self.source_tag {
            s.serialize_uint32("SourceTag", source_tag)?;
        }
        if let Some(sequence) = self.sequence {
            s.serialize_uint32("Sequence", sequence)?;
        }
        if let Some(last_ledger_sequence) = self.last_ledger_sequence {
            s.serialize_uint32("LastLedgerSequence", last_ledger_sequence)?;
        }
        if !self.memos.is_empty() {
            let mut array = s.serialize_array("Memos")?;
            for memo in &self.memos {
                array.serialize_object("Memo", memo)?;
            }
            array.end()?;
        }
        if let Some(ticket_sequence) = self.ticket_sequence {
            s.serialize_uint32("TicketSequence", ticket_sequence)?;
        }
        if let Some(account_txn_id) = self.account_txn_id {
            s.serialize_hash256("AccountTxnID", account_txn_id)?;
        }
        if let Some(fee) = self.fee {
            s.serialize_amount("Fee", Amount::Drops(fee))?;
        }
        if let Some(signing_pub_key) = self.signing_pub_key.as_ref() {
            s.serialize_blob("SigningPubKey", signing_pub_key)?;
        }
        if let Some(txn_signature) = self.txn_signature.as_ref() {
            s.serialize_blob("TxnSignature", txn_signature)?;
        }
        s.serialize_account_id("Account", self.account)?;
        Ok(())
    }
}

impl Serialize for Memo {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_blob("MemoType", &self.memo_type)?;
        s.serialize_blob("MemoData", &self.memo_data)?;
        if let Some(memo_format) = self.memo_format.as_ref() {
            s.serialize_blob("MemoFormat", memo_format)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct TransactionCommonVisitor {
    pub account: Option<AccountId>,
    pub fee: Option<DropsAmount>,
    pub sequence: Option<UInt32>,
    pub account_txn_id: Option<Hash256>,
    pub last_ledger_sequence: Option<UInt32>,
    pub memos: Vec<Memo>,
    pub network_id: Option<UInt32>,
    pub source_tag: Option<UInt32>,
    pub signing_pub_key: Option<Blob>,
    pub ticket_sequence: Option<UInt32>,
    pub txn_signature: Option<Blob>,
}

impl deserialize::Visitor for TransactionCommonVisitor {
    fn visit_field<E: DeserError, F: FieldAccessor<Error = E>>(
        &mut self,
        field_name: &str,
        mut field_accessor: F,
    ) -> Result<(), E> {
        match field_name {
            "NetworkID" => {
                self.network_id = Some(field_accessor.deserialize_uint32()?);
            }
            "SourceTag" => {
                self.source_tag = Some(field_accessor.deserialize_uint32()?);
            }
            "Sequence" => {
                self.sequence = Some(field_accessor.deserialize_uint32()?);
            }
            "LastLedgerSequence" => {
                self.last_ledger_sequence = Some(field_accessor.deserialize_uint32()?);
            }
            // todo allan memos
            "TicketSequence" => {
                self.ticket_sequence = Some(field_accessor.deserialize_uint32()?);
            }
            "AccountTxnID" => {
                self.account_txn_id = Some(field_accessor.deserialize_hash256()?);
            }
            "Fee" => {
                self.fee = Some(match field_accessor.deserialize_amount()? {
                    Amount::Issued(_) => {
                        return Err(E::invalid_value("Fee amount issued token"));
                    }
                    Amount::Drops(drops) => drops,
                });
            }
            "SigningPubKey" => {
                self.signing_pub_key = Some(field_accessor.deserialize_blob()?);
            }
            "TxnSignature" => {
                self.txn_signature = Some(field_accessor.deserialize_blob()?);
            }
            "Account" => {
                self.account = Some(field_accessor.deserialize_account_id()?);
            }
            _ => (),
        }
        Ok(())
    }
}

impl TransactionCommonVisitor {
    pub fn into_transaction_common<E: DeserError>(self) -> Result<TransactionCommon, E> {
        Ok(TransactionCommon {
            account: E::unwrap_field_value("Account", self.account)?,
            fee: self.fee,
            sequence: self.sequence,
            account_txn_id: self.account_txn_id,
            last_ledger_sequence: self.last_ledger_sequence,
            memos: self.memos,
            network_id: self.network_id,
            source_tag: self.source_tag,
            signing_pub_key: self.signing_pub_key,
            ticket_sequence: self.ticket_sequence,
            txn_signature: self.txn_signature,
        })
    }
}

impl Deserialize for TransactionCommon {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        let mut visitor = TransactionCommonVisitor::default();
        deserializer.deserialize(&mut visitor)?;
        visitor.into_transaction_common()
    }
}
