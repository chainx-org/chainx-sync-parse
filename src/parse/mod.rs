mod btc;
mod btree_map;
mod linked_node;
mod primitives;

use parity_codec::Decode;
use strum::{EnumMessage, IntoEnumIterator};

use self::btree_map::CodecBTreeMap;
use self::linked_node::{MultiNodeIndex, Node};
use self::primitives::*;
use crate::Result;

#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq, Debug, EnumIter, EnumMessage)]
pub enum RuntimeStorage {
    // ============================================================================================
    // Substrate
    // ============================================================================================
    // system -------------------------------------------------------------------------------------
    #[strum(message = "System AccountNonce", detailed_message = "map")]
    SystemAccountNonce(AccountId, Index),
    #[strum(message = "System BlockHash", detailed_message = "map")]
    SystemBlockHash(BlockNumber, Hash),
    // balances -----------------------------------------------------------------------------------
    #[strum(message = "Balances TotalIssuance", detailed_message = "value")]
    BalancesTotalIssuance(Balance),
    #[strum(message = "Balances ExistentialDeposit", detailed_message = "value")]
    BalancesExistentialDeposit(Balance),
    #[strum(message = "Balances TransferFee", detailed_message = "value")]
    BalancesTransferFee(Balance),
    #[strum(message = "Balances CreationFee", detailed_message = "value")]
    BalancesCreationFee(Balance),
    #[strum(message = "Balances FreeBalance", detailed_message = "map")]
    BalancesFreeBalance(AccountId, Balance),
    #[strum(message = "Balances ReservedBalance", detailed_message = "map")]
    BalancesReservedBalance(AccountId, Balance),
    #[strum(message = "Balances TransactionBaseFee", detailed_message = "value")]
    BalancesTransactionBaseFee(Balance),
    #[strum(message = "Balances TransactionByteFee", detailed_message = "value")]
    BalancesTransactionByteFee(Balance),
    // indices ------------------------------------------------------------------------------------
    #[strum(message = "Indices NextEnumSet", detailed_message = "value")]
    IndicesNextEnumSet(AccountIndex),
    #[strum(message = "Indices EnumSet", detailed_message = "map")]
    IndicesEnumSet(AccountIndex, Vec<AccountId>),
    // timestamp ----------------------------------------------------------------------------------
    #[strum(message = "Timestamp Now", detailed_message = "value")]
    TimestampNow(Moment),
    #[strum(message = "Timestamp BlockPeriod", detailed_message = "value")]
    TimestampBlockPeriod(Moment),
    // session ------------------------------------------------------------------------------------
    #[strum(message = "Session Validators", detailed_message = "value")]
    SessionValidators(Vec<(AccountId, u64)>),   // SessionValidators(Vec<AccountId>) in substrate
    #[strum(message = "Session SessionLength", detailed_message = "value")]
    SessionSessionLength(BlockNumber),
    #[strum(message = "Session CurrentIndex", detailed_message = "value")]
    SessionCurrentIndex(BlockNumber),
    #[strum(message = "Session CurrentStart", detailed_message = "value")]
    SessionCurrentStart(Moment),
    #[strum(message = "Session ForcingNewSession", detailed_message = "value")]
    SessionForcingNewSession(bool),
    #[strum(message = "Session NextKeyFor", detailed_message = "map")]
    SessionNextKeyFor(AccountId, SessionKey),
    // ============================================================================================
    // ChainX
    // ============================================================================================
    // xsystem ------------------------------------------------------------------------------------
    #[strum(message = "XSystem BlockProducer", detailed_message = "value")]
    XSystemBlockProducer(AccountId),
    #[strum(message = "XSystem DeathAccount", detailed_message = "value")]
    XSystemDeathAccount(AccountId),
    #[strum(message = "XSystem BannedAccount", detailed_message = "value")]
    XSystemBannedAccount(AccountId),
    #[strum(message = "XSystem BurnAccount", detailed_message = "value")]
    XSystemBurnAccount(AccountId),
    // xaccounts ----------------------------------------------------------------------------------
    #[strum(message = "XAccounts SharesPerCert", detailed_message = "value")]
    XAccountsSharesPerCert(u32),
    #[strum(message = "XAccounts ActivationPerShare", detailed_message = "value")]
    XAccountsActivationPerShare(u32),
    #[strum(message = "XAccounts MaximumCertCount", detailed_message = "value")]
    XAccountsMaximumCertCount(u32),
    #[strum(message = "XAccounts TotalIssued", detailed_message = "value")]
    XAccountsTotalIssued(u32),
    #[strum(message = "XAccounts CertOwnerOf", detailed_message = "map")]
    XAccountsCertOwnerOf(Name, AccountId),
    #[strum(message = "XAccounts CertImmutablePropertiesOf", detailed_message = "map")]
    XAccountsCertImmutablePropertiesOf(Name, CertImmutableProps<BlockNumber, Moment>),
    #[strum(message = "XAccounts RemainingSharesOf", detailed_message = "map")]
    XAccountsRemainingSharesOf(Name, u32),
    #[strum(message = "XAccounts CertNamesOf", detailed_message = "map")]
    XAccountsCertNamesOf(AccountId, Vec<Name>),
    #[strum(message = "XAccounts IntentionOf", detailed_message = "map")]
    XAccountsIntentionOf(Name, AccountId),
    #[strum(message = "XAccounts IntentionImmutablePropertiesOf", detailed_message = "map")]
    XAccountsIntentionImmutablePropertiesOf(AccountId, IntentionImmutableProps),
    #[strum(message = "XAccounts IntentionPropertiesOf", detailed_message = "map")]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps),
    // xfee ---------------------------------------------------------------------------------------
    #[strum(message = "XFeeManager Switch", detailed_message = "value")]
    XFeeManagerSwitch(bool),
    // xassets ------------------------------------------------------------------------------------
    // XAssets
    #[strum(message = "XAssets AssetList", detailed_message = "map")]
    XAssetsAssetList(Chain, Vec<Token>),
    #[strum(message = "XAssets AssetInfo", detailed_message = "map")]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
    #[strum(message = "XAssets CrossChainAssetsOf", detailed_message = "map")]
    XAssetsCrossChainAssetsOf(AccountId, Vec<Token>),
    #[strum(message = "XAssets AssetBalance", detailed_message = "map")]
    XAssetsAssetBalance((AccountId, Token), CodecBTreeMap<AssetType, Balance>),
    #[strum(message = "XAssets TotalAssetBalance", detailed_message = "map")]
    XAssetsTotalAssetBalance(Token, CodecBTreeMap<AssetType, Balance>),
    #[strum(message = "XAssets MemoLen", detailed_message = "value")]
    XAssetsMemoLen(u32),
    // XAssetsRecords
    #[strum(message = "XAssetsRecords ApplicationMHeader", detailed_message = "map")]
    XAssetsRecordsApplicationMHeader(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Moment>>),
    #[strum(message = "XAssetsRecords ApplicationMTail", detailed_message = "map")]
    XAssetsRecordsApplicationMTail(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Moment>>),
    #[strum(message = "XAssetsRecords ApplicationMap", detailed_message = "map")]
    XAssetsRecordsApplicationMap(u32, Node<Application<AccountId, Balance, Moment>>),
    #[strum(message = "XAssetsRecords SerialNumber", detailed_message = "value")]
    XAssetsRecordsSerialNumber(u32),
    // xmining ------------------------------------------------------------------------------------
    // XStaking
    #[strum(message = "XStaking ValidatorCount", detailed_message = "value")]
    XStakingValidatorCount(u32),
    #[strum(message = "XStaking MinimumValidatorCount", detailed_message = "value")]
    XStakingMinimumValidatorCount(u32),
    #[strum(message = "XStaking SessionsPerEra", detailed_message = "value")]
    XStakingSessionsPerEra(BlockNumber),
    #[strum(message = "XStaking BondingDuration", detailed_message = "value")]
    XStakingBondingDuration(BlockNumber),
    #[strum(message = "XStaking ValidatorStakeThreshold", detailed_message = "value")]
    XStakingValidatorStakeThreshold(Balance),
    #[strum(message = "XStaking CurrentEra", detailed_message = "value")]
    XStakingCurrentEra(BlockNumber),
    #[strum(message = "XStaking Intentions", detailed_message = "value")]
    XStakingIntentions(Vec<AccountId>),
    #[strum(message = "XStaking NextSessionsPerEra", detailed_message = "value")]
    XStakingNextSessionsPerEra(BlockNumber),
    #[strum(message = "XStaking LastEraLengthChange", detailed_message = "value")]
    XStakingLastEraLengthChange(BlockNumber),
    #[strum(message = "XStaking ForcingNewEra", detailed_message = "value")]
    XStakingForcingNewEra(()),
    #[strum(message = "XStaking StakeWeight", detailed_message = "map")]
    XStakingStakeWeight(AccountId, Balance),
    #[strum(message = "XStaking IntentionProfiles", detailed_message = "map")]
    XStakingIntentionProfiles(AccountId, IntentionProfs<Balance, BlockNumber>),
    #[strum(message = "XStaking NominationRecords", detailed_message = "map")]
    XStakingNominationRecords((AccountId, AccountId), NominationRecord<Balance, BlockNumber>),
    #[strum(message = "XStaking Funding", detailed_message = "value")]
    XStakingFunding(AccountId),
    #[strum(message = "XStaking Penalty", detailed_message = "value")]
    XStakingPenalty(Balance),
    #[strum(message = "XStaking PunishList", detailed_message = "value")]
    XStakingPunishList(Vec<AccountId>),
    // XTokens
    #[strum(message = "XTokens TokenDiscount", detailed_message = "value")]
    XTokensTokenDiscount(Permill),
    #[strum(message = "XTokens PseduIntentions", detailed_message = "value")]
    XTokensPseduIntentions(Vec<Token>),
    #[strum(message = "XTokens PseduIntentionProfiles", detailed_message = "map")]
    XTokensPseduIntentionProfiles(Token, PseduIntentionVoteWeight<BlockNumber>),
    #[strum(message = "XTokens DepositRecords", detailed_message = "map")]
    XTokensDepositRecords((AccountId, Token), DepositVoteWeight<BlockNumber>),
    // xdex ---------------------------------------------------------------------------------------
    // XSpot
    #[strum(message = "XSpot OrderPairLen", detailed_message = "value")]
    XSpotOrderPairLen(OrderPairID),
    #[strum(message = "XSpot OrderPairOf", detailed_message = "map")]
    XSpotOrderPairOf(OrderPairID, OrderPair),
    #[strum(message = "XSpot OrderPairPriceOf", detailed_message = "map")]
    XSpotOrderPairPriceOf(OrderPairID, (Price, Price, BlockNumber)),
    #[strum(message = "XSpot FillLen", detailed_message = "map")]
    XSpotFillLen(OrderPairID, ID),
    #[strum(message = "XSpot AccountOrdersLen", detailed_message = "map")]
    XSpotAccountOrdersLen(AccountId, ID),
    #[strum(message = "XSpot AccountOrder", detailed_message = "map")]
    XSpotAccountOrder((AccountId, ID), Order<OrderPairID, AccountId, Amount, Price, BlockNumber>),
    #[strum(message = "XSpot Quotations", detailed_message = "map")]
    XSpotQuotations((OrderPairID, Price), Vec<(AccountId, ID)>),
    #[strum(message = "XSpot HandicapMap", detailed_message = "map")]
    XSpotHandicapMap(OrderPairID, Handicap<Price>),
    #[strum(message = "XSpot PriceVolatility", detailed_message = "value")]
    XSpotPriceVolatility(u32),
    // xbridge ------------------------------------------------------------------------------------
    // BridgeOfBTC
    #[strum(message = "BridgeOfBTC BestIndex", detailed_message = "value")]
    BridgeOfBTCBestIndex(H256),
    #[strum(message = "BridgeOfBTC BlockHeaderFor", detailed_message = "map")]
    BridgeOfBTCBlockHeaderFor(H256, BlockHeaderInfo),
    #[strum(message = "BridgeOfBTC BlockHeightFor", detailed_message = "map")]
    BridgeOfBTCBlockHeightFor(u32, Vec<H256>),
    #[strum(message = "BridgeOfBTC TxFor", detailed_message = "map")]
    BridgeOfBTCTxFor(H256, TxInfo),
    #[strum(message = "BridgeOfBTC GenesisInfo", detailed_message = "value")]
    BridgeOfBTCGenesisInfo((BlockHeader, u32)),
    #[strum(message = "BridgeOfBTC ParamsInfo", detailed_message = "value")]
    BridgeOfBTCParamsInfo(Params),
    #[strum(message = "BridgeOfBTC NetworkId", detailed_message = "value")]
    BridgeOfBTCNetworkId(u32),
    #[strum(message = "BridgeOfBTC TrusteeAddress", detailed_message = "value")]
    BridgeOfBTCTrusteeAddress(btc::Address),
    #[strum(message = "BridgeOfBTC TrusteeRedeemScript", detailed_message = "value")]
    BridgeOfBTCTrusteeRedeemScript(Vec<u8>),
    #[strum(message = "BridgeOfBTC CertAddress", detailed_message = "value")]
    BridgeOfBTCCertAddress(btc::Address),
    #[strum(message = "BridgeOfBTC CertRedeemScript", detailed_message = "value")]
    BridgeOfBTCCertRedeemScript(Vec<u8>),
    #[strum(message = "BridgeOfBTC UTXOSet", detailed_message = "map")]
    BridgeOfBTCUTXOSet(UTXOKey, UTXOStatus),
    #[strum(message = "BridgeOfBTC UTXOSetKey", detailed_message = "value")]
    BridgeOfBTCUTXOSetKey(Vec<UTXOKey>),
    #[strum(message = "BridgeOfBTC ReservedBlock", detailed_message = "value")]
    BridgeOfBTCReservedBlock(u32),
    #[strum(message = "BridgeOfBTC IrrBlock", detailed_message = "value")]
    BridgeOfBTCIrrBlock(u32),
    #[strum(message = "BridgeOfBTC BtcFee", detailed_message = "value")]
    BridgeOfBTCBtcFee(u64),
    #[strum(message = "BridgeOfBTC MaxWithdrawAmount", detailed_message = "value")]
    BridgeOfBTCMaxWithdrawAmount(u32),
    #[strum(message = "BridgeOfBTC TxProposal", detailed_message = "value")]
    BridgeOfBTCTxProposal(CandidateTx),
    #[strum(message = "BridgeOfBTC PendingDepositMap", detailed_message = "map")]
    BridgeOfBTCPendingDepositMap(btc::Address, Vec<UTXOKey>),
    #[strum(message = "BridgeOfBTC AddressMap", detailed_message = "map")]
    BridgeOfBTCAddressMap(btc::Address, AccountId),
    #[strum(message = "BridgeOfBTC AccountMap", detailed_message = "map")]
    BridgeOfBTCAccountMap(AccountId, Vec<btc::Address>),
}

macro_rules! build_json {
    ($type:expr, $prefix:ident, $key:ident, $value:ident) => {
        json!({
            "type":$type,
            "prefix":$prefix,
            "key":$key,
            "value":$value,
        })
    };
}

macro_rules! to_value_json {
    ($prefix:ident, $value:ident => $v:ident) => {
        to_value_json_impl!("value", $prefix, null, $value => $v)
    };
}

macro_rules! to_map_json {
    ($prefix:ident, $key:ident => $k:ident, $value:ident => $v:ident) => {{
        *$k = match Decode::decode(&mut $key) {
            Some(key) => key,
            None => {
                let err = format!("Decode failed, prefix: {:?}, key: {:?}", $prefix, $k);
                error!("Runtime storage parse: {}", err);
                return Err(err.into());
            }
        };
        to_value_json_impl!("map", $prefix, $k, $value => $v)
    }};
}

macro_rules! to_value_json_impl {
    ($type:expr, $prefix:ident, $k:ident, $value:ident => $v:ident) => {{
        if $value.is_empty() {
            info!("Empty Value: [{:?}] may have been removed", $prefix);
            return Ok(build_json!($type, $prefix, $k, null));
        }
        *$v = match Decode::decode(&mut $value.as_slice()) {
            Some(value) => value,
            None => {
                let err = format!("Decode failed, prefix: {:?}, value: {:?}", $prefix, $v);
                error!("Runtime storage parse: {}", err);
                return Err(err.into());
            }
        };
        Ok(build_json!($type, $prefix, $k, $v))
    }};
}

impl RuntimeStorage {
    pub fn parse(key: &[u8], value: Vec<u8>) -> Result<(String, serde_json::Value)> {
        let (mut storage, prefix) = Self::match_prefix(key)?;
        let json = storage.decode_by_type(&prefix, key, value)?;
        Ok((prefix, json))
    }

    fn match_prefix(key: &[u8]) -> Result<(Self, String)> {
        for storage in Self::iter() {
            let prefix: String = match storage.get_message() {
                Some(prefix) => prefix.to_string(),
                None => {
                    error!("Runtime storage parse: get storage prefix failed");
                    return Err("Runtime storage parse: get storage prefix failed".into());
                }
            };
            if key.starts_with(prefix.as_bytes()) {
                return Ok((storage, prefix));
            }
        }
        debug!("Runtime storage parse: No matching key found");
        Err("No matching key found".into())
    }

    fn match_key<'a>(&mut self, prefix: &str, key: &'a [u8]) -> Result<&'a [u8]> {
        let key = match self.get_detailed_message() {
            Some("map") => &key[prefix.len()..],
            Some("value") => key,
            _ => {
                error!("Runtime storage parse: get storage type failed");
                return Err("Invalid storage type".into());
            }
        };
        Ok(key)
    }

    #[rustfmt::skip]
    #[allow(clippy::cyclomatic_complexity)] // cyclomatic_complexity = 234 (defaults to 25)
    fn decode_by_type(&mut self, prefix: &str, key: &[u8], value: Vec<u8>) -> Result<serde_json::Value> {
        let mut key = self.match_key(prefix, key)?;

        match self {
            // Substrate
            RuntimeStorage::SystemAccountNonce(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::SystemBlockHash(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesTotalIssuance(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesExistentialDeposit(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTransferFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesCreationFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesFreeBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesReservedBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesTransactionBaseFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTransactionByteFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::IndicesNextEnumSet(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::IndicesEnumSet(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::TimestampNow(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::TimestampBlockPeriod(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionValidators(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionSessionLength(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentStart(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionForcingNewSession(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionNextKeyFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            // ChainX
            RuntimeStorage::XSystemBlockProducer(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemDeathAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemBannedAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemBurnAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsSharesPerCert(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsActivationPerShare(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsMaximumCertCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsTotalIssued(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsCertOwnerOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCertImmutablePropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsRemainingSharesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCertNamesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionImmutablePropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XFeeManagerSwitch(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsAssetList(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsCrossChainAssetsOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsTotalAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsMemoLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMHeader(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMTail(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsSerialNumber(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingMinimumValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingValidatorStakeThreshold(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCurrentEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingNextSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingLastEraLengthChange(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingForcingNewEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingStakeWeight(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingNominationRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingFunding(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingPenalty(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingPunishList(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XTokensTokenDiscount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XTokensPseduIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XTokensPseduIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensDepositRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotOrderPairLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSpotOrderPairOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotOrderPairPriceOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotFillLen(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotAccountOrdersLen(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotAccountOrder(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotQuotations(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotHandicapMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotPriceVolatility(ref mut v) => to_value_json!(prefix, value => v),
            // bridge - bitcoin
            RuntimeStorage::BridgeOfBTCBestIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCBlockHeightFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCTxFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCGenesisInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCParamsInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCNetworkId(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCTrusteeAddress(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCTrusteeRedeemScript(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCCertAddress(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCCertRedeemScript(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCUTXOSet(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCUTXOSetKey(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCReservedBlock(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCIrrBlock(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCBtcFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCMaxWithdrawAmount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCTxProposal(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BridgeOfBTCPendingDepositMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCAddressMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BridgeOfBTCAccountMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    pub fn test_parse_match_value() {
        let key = "Balances TotalIssuance".as_bytes();
        let value = vec![123u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"value",
                "prefix":"Balances TotalIssuance",
                "key":null,
                "value":123
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    pub fn test_parse_match_map() {
        let key = "Balances FreeBalance12345678901234567890123456789012".as_bytes();
        let value = vec![123u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"map",
                "prefix":"Balances FreeBalance",
                "key":"0x3132333435363738393031323334353637383930313233343536373839303132",
                "value":123
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    pub fn test_parse_match_map_option() {
        let key = "XAssets AssetInfo\u{c}PCX".as_bytes();
        let value = vec![
            12, 80, 67, 88, 56, 80, 111, 108, 107, 97, 100, 111, 116, 67, 104, 97, 105, 110, 88, 0,
            3, 0, 68, 80, 67, 88, 32, 111, 110, 99, 104, 97, 105, 110, 32, 116, 111, 107, 101, 110,
            1, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"map",
                "prefix":"XAssets AssetInfo",
                "key":"PCX",
                "value":[
                    {
                        "token":"PCX",
                        "token_name":"PolkadotChainX",
                        "chain":"ChainX",
                        "precision":3,
                        "desc":"PCX onchain token"
                    },
                    true,
                    0
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    pub fn test_parse_match_codec_btree_map() {
        let key = "XAssets TotalAssetBalance\u{c}BTC".as_bytes();
        let value = vec![1, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0];
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"map",
                "prefix":"XAssets TotalAssetBalance",
                "key":"BTC",
                "value":{"Free":123}
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    pub fn test_parse_remove_value() {
        let key = "XSystem BlockProducer".as_bytes();
        let value = vec![];
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"value",
                "prefix":"XSystem BlockProducer",
                "key":null,
                "value":null
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }
}
