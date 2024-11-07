use crate::alloc::string::{String, ToString};

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
use crate::field::FieldId;

#[cfg(feature = "std")]
static FIELD_NAME_TO_FIELD_ID: std::sync::OnceLock<HashMap<String, FieldId>> = std::sync::OnceLock::new();

#[cfg(feature = "std")]
pub fn field_name_to_field_id() -> &'static HashMap<String, FieldId> {
    FIELD_NAME_TO_FIELD_ID.get_or_init(create_field_name_to_field_id_map)
}

#[cfg(not(feature = "std"))]
static FIELD_NAME_TO_FIELD_ID: spin::Once<HashMap<String, FieldId>> = spin::Once::new();

#[cfg(not(feature = "std"))]
pub fn field_name_to_field_id() -> &'static HashMap<String, FieldId> {
    FIELD_NAME_TO_FIELD_ID.call_once(|| create_field_name_to_field_id_map())
}

pub fn field_id_by_name(field_name: &str) -> Option<&'static FieldId> {
    field_name_to_field_id().get(field_name)
}

#[cfg(feature = "std")]
static FIELD_ID_TO_FIELD_NAME: std::sync::OnceLock<HashMap<FieldId, String>> = std::sync::OnceLock::new();

#[cfg(feature = "std")]
pub fn field_id_to_field_name() -> &'static HashMap<FieldId, String> {
    FIELD_ID_TO_FIELD_NAME.get_or_init(|| create_field_id_to_field_name_map())
}

#[cfg(not(feature = "std"))]
static FIELD_ID_TO_FIELD_NAME: spin::Once<HashMap<FieldId, String>> = spin::Once::new();

#[cfg(not(feature = "std"))]
pub fn field_id_to_field_name() -> &'static HashMap<FieldId, String> {
    FIELD_ID_TO_FIELD_NAME.call_once(|| create_field_id_to_field_name_map())
}

pub fn field_name_by_id(field_id: FieldId) -> Option<&'static str> {
    field_id_to_field_name().get(&field_id).map(|s| s.as_str())
}

fn create_field_id_to_field_name_map() -> HashMap<FieldId, String> {
    field_name_to_field_id().clone().into_iter().map(|(k, v)|(v, k)).collect()
}


macro_rules! insert_field_by_name {
    ($map:ident, $field_name:literal, $field_code:literal, $field_type:ident) => {
        if $map
            .insert(
                $field_name.to_string(),
                FieldId {
                    type_code: $crate::field::TypeCode::$field_type,
                    field_code: $crate::field::FieldCode($field_code),
                },
            )
            .is_some()
        {
            panic!("Field with name {} inserted twice", $field_name);
        }
    };
}

/// Field info taken from FIELDS in <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json>
fn create_field_name_to_field_id_map() -> HashMap<String, FieldId> {
    let mut map = HashMap::new();
    insert_field_by_name!(map, "CloseResolution", 1, UInt8);
    insert_field_by_name!(map, "Method", 2, UInt8);
    insert_field_by_name!(map, "TransactionResult", 3, UInt8);
    insert_field_by_name!(map, "TickSize", 16, UInt8);
    insert_field_by_name!(map, "UNLModifyDisabling", 17, UInt8);
    insert_field_by_name!(map, "HookResult", 18, UInt8);
    insert_field_by_name!(map, "LedgerEntryType", 1, UInt16);
    insert_field_by_name!(map, "TransactionType", 2, UInt16);
    insert_field_by_name!(map, "SignerWeight", 3, UInt16);
    insert_field_by_name!(map, "TransferFee", 4, UInt16);
    insert_field_by_name!(map, "Version", 16, UInt16);
    insert_field_by_name!(map, "HookStateChangeCount", 17, UInt16);
    insert_field_by_name!(map, "HookEmitCount", 18, UInt16);
    insert_field_by_name!(map, "HookExecutionIndex", 19, UInt16);
    insert_field_by_name!(map, "HookApiVersion", 20, UInt16);
    insert_field_by_name!(map, "NetworkID", 1, UInt32);
    insert_field_by_name!(map, "Flags", 2, UInt32);
    insert_field_by_name!(map, "SourceTag", 3, UInt32);
    insert_field_by_name!(map, "Sequence", 4, UInt32);
    insert_field_by_name!(map, "PreviousTxnLgrSeq", 5, UInt32);
    insert_field_by_name!(map, "LedgerSequence", 6, UInt32);
    insert_field_by_name!(map, "CloseTime", 7, UInt32);
    insert_field_by_name!(map, "ParentCloseTime", 8, UInt32);
    insert_field_by_name!(map, "SigningTime", 9, UInt32);
    insert_field_by_name!(map, "Expiration", 10, UInt32);
    insert_field_by_name!(map, "TransferRate", 11, UInt32);
    insert_field_by_name!(map, "WalletSize", 12, UInt32);
    insert_field_by_name!(map, "OwnerCount", 13, UInt32);
    insert_field_by_name!(map, "DestinationTag", 14, UInt32);
    insert_field_by_name!(map, "HighQualityIn", 16, UInt32);
    insert_field_by_name!(map, "HighQualityOut", 17, UInt32);
    insert_field_by_name!(map, "LowQualityIn", 18, UInt32);
    insert_field_by_name!(map, "LowQualityOut", 19, UInt32);
    insert_field_by_name!(map, "QualityIn", 20, UInt32);
    insert_field_by_name!(map, "QualityOut", 21, UInt32);
    insert_field_by_name!(map, "StampEscrow", 22, UInt32);
    insert_field_by_name!(map, "BondAmount", 23, UInt32);
    insert_field_by_name!(map, "LoadFee", 24, UInt32);
    insert_field_by_name!(map, "OfferSequence", 25, UInt32);
    insert_field_by_name!(map, "FirstLedgerSequence", 26, UInt32);
    insert_field_by_name!(map, "LastLedgerSequence", 27, UInt32);
    insert_field_by_name!(map, "TransactionIndex", 28, UInt32);
    insert_field_by_name!(map, "OperationLimit", 29, UInt32);
    insert_field_by_name!(map, "ReferenceFeeUnits", 30, UInt32);
    insert_field_by_name!(map, "ReserveBase", 31, UInt32);
    insert_field_by_name!(map, "ReserveIncrement", 32, UInt32);
    insert_field_by_name!(map, "SetFlag", 33, UInt32);
    insert_field_by_name!(map, "ClearFlag", 34, UInt32);
    insert_field_by_name!(map, "SignerQuorum", 35, UInt32);
    insert_field_by_name!(map, "CancelAfter", 36, UInt32);
    insert_field_by_name!(map, "FinishAfter", 37, UInt32);
    insert_field_by_name!(map, "SignerListID", 38, UInt32);
    insert_field_by_name!(map, "SettleDelay", 39, UInt32);
    insert_field_by_name!(map, "TicketCount", 40, UInt32);
    insert_field_by_name!(map, "TicketSequence", 41, UInt32);
    insert_field_by_name!(map, "NFTokenTaxon", 42, UInt32);
    insert_field_by_name!(map, "MintedNFTokens", 43, UInt32);
    insert_field_by_name!(map, "BurnedNFTokens", 44, UInt32);
    insert_field_by_name!(map, "HookStateCount", 45, UInt32);
    insert_field_by_name!(map, "EmitGeneration", 46, UInt32);
    insert_field_by_name!(map, "IndexNext", 1, UInt64);
    insert_field_by_name!(map, "IndexPrevious", 2, UInt64);
    insert_field_by_name!(map, "BookNode", 3, UInt64);
    insert_field_by_name!(map, "OwnerNode", 4, UInt64);
    insert_field_by_name!(map, "BaseFee", 5, UInt64);
    insert_field_by_name!(map, "ExchangeRate", 6, UInt64);
    insert_field_by_name!(map, "LowNode", 7, UInt64);
    insert_field_by_name!(map, "HighNode", 8, UInt64);
    insert_field_by_name!(map, "DestinationNode", 9, UInt64);
    insert_field_by_name!(map, "Cookie", 10, UInt64);
    insert_field_by_name!(map, "ServerVersion", 11, UInt64);
    insert_field_by_name!(map, "NFTokenOfferNode", 12, UInt64);
    insert_field_by_name!(map, "EmitBurden", 13, UInt64);
    insert_field_by_name!(map, "HookOn", 16, UInt64);
    insert_field_by_name!(map, "HookInstructionCount", 17, UInt64);
    insert_field_by_name!(map, "HookReturnCode", 18, UInt64);
    insert_field_by_name!(map, "ReferenceCount", 19, UInt64);
    insert_field_by_name!(map, "EmailHash", 1, Hash128);
    insert_field_by_name!(map, "TakerPaysCurrency", 1, Hash160);
    insert_field_by_name!(map, "TakerPaysIssuer", 2, Hash160);
    insert_field_by_name!(map, "TakerGetsCurrency", 3, Hash160);
    insert_field_by_name!(map, "TakerGetsIssuer", 4, Hash160);
    insert_field_by_name!(map, "LedgerHash", 1, Hash256);
    insert_field_by_name!(map, "ParentHash", 2, Hash256);
    insert_field_by_name!(map, "TransactionHash", 3, Hash256);
    insert_field_by_name!(map, "AccountHash", 4, Hash256);
    insert_field_by_name!(map, "PreviousTxnID", 5, Hash256);
    insert_field_by_name!(map, "LedgerIndex", 6, Hash256);
    insert_field_by_name!(map, "WalletLocator", 7, Hash256);
    insert_field_by_name!(map, "RootIndex", 8, Hash256);
    insert_field_by_name!(map, "AccountTxnID", 9, Hash256);
    insert_field_by_name!(map, "NFTokenID", 10, Hash256);
    insert_field_by_name!(map, "EmitParentTxnID", 11, Hash256);
    insert_field_by_name!(map, "EmitNonce", 12, Hash256);
    insert_field_by_name!(map, "EmitHookHash", 13, Hash256);
    insert_field_by_name!(map, "BookDirectory", 16, Hash256);
    insert_field_by_name!(map, "InvoiceID", 17, Hash256);
    insert_field_by_name!(map, "Nickname", 18, Hash256);
    insert_field_by_name!(map, "Amendment", 19, Hash256);
    insert_field_by_name!(map, "Digest", 21, Hash256);
    insert_field_by_name!(map, "Channel", 22, Hash256);
    insert_field_by_name!(map, "ConsensusHash", 23, Hash256);
    insert_field_by_name!(map, "CheckID", 24, Hash256);
    insert_field_by_name!(map, "ValidatedHash", 25, Hash256);
    insert_field_by_name!(map, "PreviousPageMin", 26, Hash256);
    insert_field_by_name!(map, "NextPageMin", 27, Hash256);
    insert_field_by_name!(map, "NFTokenBuyOffer", 28, Hash256);
    insert_field_by_name!(map, "NFTokenSellOffer", 29, Hash256);
    insert_field_by_name!(map, "HookStateKey", 30, Hash256);
    insert_field_by_name!(map, "HookHash", 31, Hash256);
    insert_field_by_name!(map, "HookNamespace", 32, Hash256);
    insert_field_by_name!(map, "HookSetTxnID", 33, Hash256);
    insert_field_by_name!(map, "Amount", 1, Amount);
    insert_field_by_name!(map, "Balance", 2, Amount);
    insert_field_by_name!(map, "LimitAmount", 3, Amount);
    insert_field_by_name!(map, "TakerPays", 4, Amount);
    insert_field_by_name!(map, "TakerGets", 5, Amount);
    insert_field_by_name!(map, "LowLimit", 6, Amount);
    insert_field_by_name!(map, "HighLimit", 7, Amount);
    insert_field_by_name!(map, "Fee", 8, Amount);
    insert_field_by_name!(map, "SendMax", 9, Amount);
    insert_field_by_name!(map, "DeliverMin", 10, Amount);
    insert_field_by_name!(map, "MinimumOffer", 16, Amount);
    insert_field_by_name!(map, "RippleEscrow", 17, Amount);
    insert_field_by_name!(map, "DeliveredAmount", 18, Amount);
    insert_field_by_name!(map, "NFTokenBrokerFee", 19, Amount);
    insert_field_by_name!(map, "PublicKey", 1, Blob);
    insert_field_by_name!(map, "MessageKey", 2, Blob);
    insert_field_by_name!(map, "SigningPubKey", 3, Blob);
    insert_field_by_name!(map, "TxnSignature", 4, Blob);
    insert_field_by_name!(map, "URI", 5, Blob);
    insert_field_by_name!(map, "Signature", 6, Blob);
    insert_field_by_name!(map, "Domain", 7, Blob);
    insert_field_by_name!(map, "FundCode", 8, Blob);
    insert_field_by_name!(map, "RemoveCode", 9, Blob);
    insert_field_by_name!(map, "ExpireCode", 10, Blob);
    insert_field_by_name!(map, "CreateCode", 11, Blob);
    insert_field_by_name!(map, "MemoType", 12, Blob);
    insert_field_by_name!(map, "MemoData", 13, Blob);
    insert_field_by_name!(map, "MemoFormat", 14, Blob);
    insert_field_by_name!(map, "Fulfillment", 16, Blob);
    insert_field_by_name!(map, "Condition", 17, Blob);
    insert_field_by_name!(map, "MasterSignature", 18, Blob);
    insert_field_by_name!(map, "UNLModifyValidator", 19, Blob);
    insert_field_by_name!(map, "ValidatorToDisable", 20, Blob);
    insert_field_by_name!(map, "ValidatorToReEnable", 21, Blob);
    insert_field_by_name!(map, "HookStateData", 22, Blob);
    insert_field_by_name!(map, "HookReturnString", 23, Blob);
    insert_field_by_name!(map, "HookParameterName", 24, Blob);
    insert_field_by_name!(map, "HookParameterValue", 25, Blob);
    insert_field_by_name!(map, "Account", 1, AccountId);
    insert_field_by_name!(map, "Owner", 2, AccountId);
    insert_field_by_name!(map, "Destination", 3, AccountId);
    insert_field_by_name!(map, "Issuer", 4, AccountId);
    insert_field_by_name!(map, "Authorize", 5, AccountId);
    insert_field_by_name!(map, "Unauthorize", 6, AccountId);
    insert_field_by_name!(map, "RegularKey", 8, AccountId);
    insert_field_by_name!(map, "NFTokenMinter", 9, AccountId);
    insert_field_by_name!(map, "EmitCallback", 10, AccountId);
    insert_field_by_name!(map, "HookAccount", 16, AccountId);
    insert_field_by_name!(map, "TransactionMetaData", 2, Object);
    insert_field_by_name!(map, "CreatedNode", 3, Object);
    insert_field_by_name!(map, "DeletedNode", 4, Object);
    insert_field_by_name!(map, "ModifiedNode", 5, Object);
    insert_field_by_name!(map, "PreviousFields", 6, Object);
    insert_field_by_name!(map, "FinalFields", 7, Object);
    insert_field_by_name!(map, "NewFields", 8, Object);
    insert_field_by_name!(map, "TemplateEntry", 9, Object);
    insert_field_by_name!(map, "Memo", 10, Object);
    insert_field_by_name!(map, "SignerEntry", 11, Object);
    insert_field_by_name!(map, "NFToken", 12, Object);
    insert_field_by_name!(map, "EmitDetails", 13, Object);
    insert_field_by_name!(map, "Hook", 14, Object);
    insert_field_by_name!(map, "Signer", 16, Object);
    insert_field_by_name!(map, "Majority", 18, Object);
    insert_field_by_name!(map, "DisabledValidator", 19, Object);
    insert_field_by_name!(map, "EmittedTxn", 20, Object);
    insert_field_by_name!(map, "HookExecution", 21, Object);
    insert_field_by_name!(map, "HookDefinition", 22, Object);
    insert_field_by_name!(map, "HookParameter", 23, Object);
    insert_field_by_name!(map, "HookGrant", 24, Object);
    insert_field_by_name!(map, "ObjectEndMarker", 1, Object);
    insert_field_by_name!(map, "Signers", 3, Array);
    insert_field_by_name!(map, "SignerEntries", 4, Array);
    insert_field_by_name!(map, "Template", 5, Array);
    insert_field_by_name!(map, "Necessary", 6, Array);
    insert_field_by_name!(map, "Sufficient", 7, Array);
    insert_field_by_name!(map, "AffectedNodes", 8, Array);
    insert_field_by_name!(map, "Memos", 9, Array);
    insert_field_by_name!(map, "NFTokens", 10, Array);
    insert_field_by_name!(map, "Hooks", 11, Array);
    insert_field_by_name!(map, "Majorities", 16, Array);
    insert_field_by_name!(map, "DisabledValidators", 17, Array);
    insert_field_by_name!(map, "HookExecutions", 18, Array);
    insert_field_by_name!(map, "HookParameters", 19, Array);
    insert_field_by_name!(map, "HookGrants", 20, Array);
    insert_field_by_name!(map, "ArrayEndMarker", 1, Array);
    map
}
