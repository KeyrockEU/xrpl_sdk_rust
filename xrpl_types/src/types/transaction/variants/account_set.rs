use alloc::{format, string::ToString, vec::Vec};
use crate::deserialize::{DeserError, Deserialize, Deserializer, FieldAccessor};
use crate::serialize::{Serialize, Serializer};
use crate::{
    deserialize, AccountId, Blob, Hash128, Hash256, TransactionCommon, TransactionCommonVisitor,
    TransactionTrait, TransactionType, UInt32, UInt8,
};
use enumflags2::{bitflags, BitFlags};

/// An `AccountSet` transaction <https://xrpl.org/accountset.html>
#[derive(Debug, Clone)]
pub struct AccountSetTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<AccountSetTransactionFlags>,
    pub clear_flag: Option<AccountSetFlag>,
    pub domain: Option<Blob>,
    pub email_hash: Option<Hash128>,
    pub message_key: Option<Blob>,
    pub nf_token_minter: Option<Blob>,
    pub set_flag: Option<AccountSetFlag>,
    pub transfer_rate: Option<UInt32>,
    pub tick_size: Option<UInt8>,
    pub wallet_locator: Option<Hash256>,
    pub wallet_size: Option<UInt32>,
}

impl AccountSetTransaction {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            nf_token_minter: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: None,
            wallet_locator: None,
            wallet_size: None,
        }
    }
}

impl TransactionTrait for AccountSetTransaction {
    fn common(&self) -> &TransactionCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut TransactionCommon {
        &mut self.common
    }
}

/// `AccountSet` flags <https://xrpl.org/accountset.html#accountset-flags>
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountSetFlag {
    AccountTxnID = 5,
    AllowTrustLineClawback = 16,
    AuthorizedNFTokenMinter = 10,
    DefaultRipple = 8,
    DepositAuth = 9,
    DisableMaster = 4,
    DisallowIncomingCheck = 13,
    DisallowIncomingNFTokenOffer = 12,
    DisallowIncomingPayChan = 14,
    DisallowIncomingTrustline = 15,
    DisallowXRP = 3,
    GlobalFreeze = 7,
    NoFreeze = 6,
    RequireAuth = 2,
    RequireDest = 1,
}

impl AccountSetFlag {
    pub fn from_discriminant_opt(disc: u32) -> Option<Self> {
        match disc {
            5 => Some(Self::AccountTxnID),
            16 => Some(Self::AllowTrustLineClawback),
            10 => Some(Self::AuthorizedNFTokenMinter),
            8 => Some(Self::DefaultRipple),
            9 => Some(Self::DepositAuth),
            4 => Some(Self::DisableMaster),
            13 => Some(Self::DisallowIncomingCheck),
            12 => Some(Self::DisallowIncomingNFTokenOffer),
            14 => Some(Self::DisallowIncomingPayChan),
            15 => Some(Self::DisallowIncomingTrustline),
            3 => Some(Self::DisallowXRP),
            7 => Some(Self::GlobalFreeze),
            6 => Some(Self::NoFreeze),
            2 => Some(Self::RequireAuth),
            1 => Some(Self::RequireDest),
            _ => None,
        }
    }
}

/// `AccountSet` flags <https://xrpl.org/accountset.html#accountset-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountSetTransactionFlags {
    FullyCanonicalSig = 0x80000000,
    RequireDestTag = 0x00010000,
    OptionalDestTag = 0x00020000,
    RequireAuth = 0x00040000,
    OptionalAuth = 0x00080000,
    DisallowXRP = 0x00100000,
    AllowXRP = 0x00200000,
}

impl Serialize for AccountSetTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::AccountSet as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        if let Some(clear_flag) = self.clear_flag {
            s.serialize_uint32("ClearFlag", clear_flag as UInt32)?;
        }
        if let Some(domain) = self.domain.as_ref() {
            s.serialize_blob("Domain", domain)?;
        }
        if let Some(email_hash) = self.email_hash {
            s.serialize_hash128("EmailHash", email_hash)?;
        }
        if let Some(message_key) = self.message_key.as_ref() {
            s.serialize_blob("MessageKey", message_key)?;
        }
        if let Some(nf_token_minter) = self.nf_token_minter.as_ref() {
            s.serialize_blob("NFTokenMinter", nf_token_minter)?;
        }
        if let Some(set_flag) = self.set_flag {
            s.serialize_uint32("SetFlag", set_flag as UInt32)?;
        }
        if let Some(transfer_rate) = self.transfer_rate {
            s.serialize_uint32("TransferRate", transfer_rate)?;
        }
        if let Some(tick_size) = self.tick_size {
            s.serialize_uint8("TickSize", tick_size)?;
        }
        if let Some(wallet_locator) = self.wallet_locator {
            s.serialize_hash256("WalletLocator", wallet_locator)?;
        }
        if let Some(wallet_size) = self.wallet_size {
            s.serialize_uint32("WalletSize", wallet_size)?;
        }
        Ok(())
    }
}

impl Deserialize for AccountSetTransaction {
    fn deserialize<S: Deserializer>(deserializer: S) -> Result<Self, S::Error>
    where
        Self: Sized,
    {
        #[derive(Default)]
        struct Visitor {
            common: TransactionCommonVisitor,
            flags: BitFlags<AccountSetTransactionFlags>,
            clear_flag: Option<AccountSetFlag>,
            domain: Option<Blob>,
            email_hash: Option<Hash128>,
            message_key: Option<Blob>,
            nf_token_minter: Option<Blob>,
            set_flag: Option<AccountSetFlag>,
            transfer_rate: Option<UInt32>,
            tick_size: Option<UInt8>,
            wallet_locator: Option<Hash256>,
            wallet_size: Option<UInt32>,
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
                            != TransactionType::AccountSet as u16
                        {
                            return Err(E::invalid_value("Wrong transaction type"));
                        }
                    }
                    "Flags" => {
                        self.flags = BitFlags::from_bits(field_accessor.deserialize_uint32()?)
                            .map_err(E::invalid_value)?;
                    }
                    "ClearFlag" => {
                        let clear_flag = field_accessor.deserialize_uint32()?;
                        self.clear_flag = Some(
                            AccountSetFlag::from_discriminant_opt(clear_flag).ok_or_else(|| {
                                E::invalid_value(format!(
                                    "Unknown account set flag: {}",
                                    clear_flag
                                ))
                            })?,
                        );
                    }
                    "Domain" => {
                        self.domain = Some(field_accessor.deserialize_blob()?);
                    }
                    "EmailHash" => {
                        self.email_hash = Some(field_accessor.deserialize_hash128()?);
                    }
                    "MessageKey" => {
                        self.message_key = Some(field_accessor.deserialize_blob()?);
                    }
                    "NFTokenMinter" => {
                        self.nf_token_minter = Some(field_accessor.deserialize_blob()?);
                    }
                    "SetFlag" => {
                        let set_flag = field_accessor.deserialize_uint32()?;
                        self.set_flag = Some(
                            AccountSetFlag::from_discriminant_opt(set_flag).ok_or_else(|| {
                                E::invalid_value(format!("Unknown account set flag: {}", set_flag))
                            })?,
                        );
                    }
                    "TransferRate" => {
                        self.transfer_rate = Some(field_accessor.deserialize_uint32()?);
                    }
                    "TickSize" => {
                        self.tick_size = Some(field_accessor.deserialize_uint8()?);
                    }
                    "WalletLocator" => {
                        self.wallet_locator = Some(field_accessor.deserialize_hash256()?);
                    }
                    "WalletSize" => {
                        self.wallet_size = Some(field_accessor.deserialize_uint32()?);
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

        Ok(AccountSetTransaction {
            common: visitor.common.into_transaction_common()?,
            flags: visitor.flags,
            clear_flag: visitor.clear_flag,
            domain: visitor.domain,
            email_hash: visitor.email_hash,
            message_key: visitor.message_key,
            nf_token_minter: visitor.nf_token_minter,
            set_flag: visitor.set_flag,
            transfer_rate: visitor.transfer_rate,
            tick_size: visitor.tick_size,
            wallet_locator: visitor.wallet_locator,
            wallet_size: visitor.wallet_size,
        })
    }
}
