mod common;
mod variants;

use alloc::{format, string::ToString, vec::Vec};
use crate::deserialize::FieldAccessor;
use crate::deserialize::{DeserError, Deserialize, Deserializer};
use crate::serialize::{Serialize};
pub use common::*;
pub use variants::*;

/// XRPL transaction
pub trait TransactionTrait: Serialize {
    fn common(&self) -> &TransactionCommon;
    fn common_mut(&mut self) -> &mut TransactionCommon;
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum TransactionType {
    // Discriminant values can be found at https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json
    Payment = 0,
    EscrowCreate = 1,
    EscrowFinish = 2,
    AccountSet = 3,
    EscrowCancel = 4,
    SetRegularKey = 5,
    NickNameSet = 6,
    OfferCreate = 7,
    OfferCancel = 8,
    Contract = 9,
    TicketCreate = 10,
    TicketCancel = 11,
    SignerListSet = 12,
    PaymentChannelCreate = 13,
    PaymentChannelFund = 14,
    PaymentChannelClaim = 15,
    CheckCreate = 16,
    CheckCash = 17,
    CheckCancel = 18,
    DepositPreauth = 19,
    TrustSet = 20,
    AccountDelete = 21,
    SetHook = 22,
    NFTokenMint = 25,
    NFTokenBurn = 26,
    NFTokenCreateOffer = 27,
    NFTokenCancelOffer = 28,
    NFTokenAcceptOffer = 29,
    Clawback = 30,
    AMMCreate = 35,
    AMMDeposit = 36,
    AMMWithdraw = 37,
    AMMVote = 38,
    AMMBid = 39,
    AMMDelete = 40,
    XChainCreateClaimID = 41,
    XChainCommit = 42,
    XChainClaim = 43,
    XChainAccountCreateCommit = 44,
    XChainAddClaimAttestation = 45,
    XChainAddAccountCreateAttestation = 46,
    XChainModifyBridge = 47,
    XChainCreateBridge = 48,
    DIDSet = 49,
    DIDDelete = 50,
    EnableAmendment = 100,
    SetFee = 101,
    UNLModify = 102,
}

impl TransactionType {
    pub fn from_discriminant_opt(disc: u16) -> Option<Self> {
        match disc {
            0 => Some(Self::Payment),
            1 => Some(Self::EscrowCreate),
            2 => Some(Self::EscrowFinish),
            3 => Some(Self::AccountSet),
            4 => Some(Self::EscrowCancel),
            5 => Some(Self::SetRegularKey),
            6 => Some(Self::NickNameSet),
            7 => Some(Self::OfferCreate),
            8 => Some(Self::OfferCancel),
            9 => Some(Self::Contract),
            10 => Some(Self::TicketCreate),
            11 => Some(Self::TicketCancel),
            12 => Some(Self::SignerListSet),
            13 => Some(Self::PaymentChannelCreate),
            14 => Some(Self::PaymentChannelFund),
            15 => Some(Self::PaymentChannelClaim),
            16 => Some(Self::CheckCreate),
            17 => Some(Self::CheckCash),
            18 => Some(Self::CheckCancel),
            19 => Some(Self::DepositPreauth),
            20 => Some(Self::TrustSet),
            21 => Some(Self::AccountDelete),
            22 => Some(Self::SetHook),
            25 => Some(Self::NFTokenMint),
            26 => Some(Self::NFTokenBurn),
            27 => Some(Self::NFTokenCreateOffer),
            28 => Some(Self::NFTokenCancelOffer),
            29 => Some(Self::NFTokenAcceptOffer),
            30 => Some(Self::Clawback),
            35 => Some(Self::AMMCreate),
            36 => Some(Self::AMMDeposit),
            37 => Some(Self::AMMWithdraw),
            38 => Some(Self::AMMVote),
            39 => Some(Self::AMMBid),
            40 => Some(Self::AMMDelete),
            41 => Some(Self::XChainCreateClaimID),
            42 => Some(Self::XChainCommit),
            43 => Some(Self::XChainClaim),
            44 => Some(Self::XChainAccountCreateCommit),
            45 => Some(Self::XChainAddClaimAttestation),
            46 => Some(Self::XChainAddAccountCreateAttestation),
            47 => Some(Self::XChainModifyBridge),
            48 => Some(Self::XChainCreateBridge),
            49 => Some(Self::DIDSet),
            50 => Some(Self::DIDDelete),
            100 => Some(Self::EnableAmendment),
            101 => Some(Self::SetFee),
            102 => Some(Self::UNLModify),
            _ => None,
        }
    }
}

/// Ledger transaction. See <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone)]
pub enum Transaction {
    AccountDelete(AccountDeleteTransaction),
    AccountSet(AccountSetTransaction),
    // TODO add model for remaining transactions
    CheckCancel(TransactionCommon),
    CheckCash(TransactionCommon),
    CheckCreate(TransactionCommon),
    DepositPreauth(TransactionCommon),
    EscrowCancel(TransactionCommon),
    EscrowCreate(TransactionCommon),
    EscrowFinish(TransactionCommon),
    NFTokenAcceptOffer(TransactionCommon),
    NFTokenBurn(TransactionCommon),
    NFTokenCancelOffer(TransactionCommon),
    NFTokenCreateOffer(TransactionCommon),
    NFTokenMint(TransactionCommon),
    OfferCancel(OfferCancelTransaction),
    OfferCreate(OfferCreateTransaction),
    Payment(PaymentTransaction),
    PaymentChannelClaim(TransactionCommon),
    PaymentChannelCreate(TransactionCommon),
    PaymentChannelFund(TransactionCommon),
    SetRegularKey(TransactionCommon),
    SignerListSet(TransactionCommon),
    TicketCreate(TransactionCommon),
    TrustSet(TrustSetTransaction),
}

impl Deserialize for Transaction {
    fn deserialize<S: Deserializer>(mut deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        let txn_type = deserializer
            .deserialize_single_field("TransactionType")?
            .deserialize_uint16()?;
        let txn_type = TransactionType::from_discriminant_opt(txn_type).ok_or_else(|| {
            S::Error::invalid_value(format!("Unknown transaction type: {}", txn_type))
        })?;
        Ok(match txn_type {
            TransactionType::Payment => {
                Self::Payment(PaymentTransaction::deserialize(deserializer)?)
            }
            TransactionType::EscrowCreate => {
                Self::EscrowCreate(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::EscrowFinish => {
                Self::EscrowFinish(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::AccountSet => {
                Self::AccountSet(AccountSetTransaction::deserialize(deserializer)?)
            }
            TransactionType::EscrowCancel => {
                Self::EscrowCancel(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::SetRegularKey => {
                Self::SetRegularKey(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::OfferCreate => {
                Self::OfferCreate(OfferCreateTransaction::deserialize(deserializer)?)
            }
            TransactionType::OfferCancel => {
                Self::OfferCancel(OfferCancelTransaction::deserialize(deserializer)?)
            }
            TransactionType::TicketCreate => {
                Self::TicketCreate(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::SignerListSet => {
                Self::SignerListSet(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::PaymentChannelCreate => {
                Self::PaymentChannelCreate(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::PaymentChannelFund => {
                Self::PaymentChannelFund(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::PaymentChannelClaim => {
                Self::PaymentChannelClaim(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::CheckCreate => {
                Self::CheckCreate(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::CheckCash => {
                Self::CheckCash(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::CheckCancel => {
                Self::CheckCancel(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::DepositPreauth => {
                Self::DepositPreauth(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::TrustSet => {
                Self::TrustSet(TrustSetTransaction::deserialize(deserializer)?)
            }
            TransactionType::AccountDelete => {
                Self::AccountDelete(AccountDeleteTransaction::deserialize(deserializer)?)
            }
            TransactionType::NFTokenMint => {
                Self::NFTokenMint(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::NFTokenBurn => {
                Self::NFTokenBurn(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::NFTokenCreateOffer => {
                Self::NFTokenCreateOffer(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::NFTokenCancelOffer => {
                Self::NFTokenCancelOffer(TransactionCommon::deserialize(deserializer)?)
            }
            TransactionType::NFTokenAcceptOffer => {
                Self::NFTokenAcceptOffer(TransactionCommon::deserialize(deserializer)?)
            }
            _ => {
                return Err(S::Error::invalid_value(format!(
                    "Unknown transaction type: {:?}",
                    txn_type
                )))
            }
        })
    }
}
