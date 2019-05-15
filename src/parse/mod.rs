mod primitives;

use std::collections::BTreeMap;

use parity_codec::Decode;
use serde_json::json;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty, IntoStaticStr};

use self::primitives::*;
use crate::types::{btc, MultiNodeIndex, Node};
use crate::Result;

#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq, Debug, IntoStaticStr, EnumIter, EnumProperty)]
pub enum RuntimeStorage {
    // ============================================================================================
    // Substrate
    // ============================================================================================
    // system -------------------------------------------------------------------------------------
//    #[strum(serialize = "System Number", props(r#type = "value"))]
//    SystemNumber(BlockNumber),
    #[strum(serialize = "System AccountNonce", props(r#type = "map"))]
    SystemAccountNonce(AccountId, Index),
    #[strum(serialize = "System BlockHash", props(r#type = "map"))]
    SystemBlockHash(BlockNumber, Hash),
    // indices ------------------------------------------------------------------------------------
    #[strum(serialize = "Indices NextEnumSet", props(r#type = "value"))]
    IndicesNextEnumSet(AccountIndex),
    #[strum(serialize = "Indices EnumSet", props(r#type = "map"))]
    IndicesEnumSet(AccountIndex, Vec<AccountId>),
    // timestamp ----------------------------------------------------------------------------------
    #[strum(serialize = "Timestamp Now", props(r#type = "value"))]
    TimestampNow(Timestamp),
    #[strum(serialize = "Timestamp BlockPeriod", props(r#type = "value"))]
    TimestampBlockPeriod(Timestamp),
    #[strum(serialize = "Timestamp MinimumPeriod", props(r#type = "value"))]
    TimestampMinimumPeriod(Timestamp),
    // finality_tracker ---------------------------------------------------------------------------
    #[strum(serialize = "Timestamp WindowSize", props(r#type = "value"))]
    TimestampWindowSize(BlockNumber),
    #[strum(serialize = "Timestamp ReportLatency", props(r#type = "value"))]
    TimestampReportLatency(BlockNumber),
    // session ------------------------------------------------------------------------------------
    #[strum(serialize = "Session Validators", props(r#type = "value"))]
    SessionValidators(Vec<(AccountId, u64)>),
    #[strum(serialize = "Session SessionLength", props(r#type = "value"))]
    SessionSessionLength(BlockNumber),
    #[strum(serialize = "Session CurrentIndex", props(r#type = "value"))]
    SessionCurrentIndex(BlockNumber),
    #[strum(serialize = "Session CurrentStart", props(r#type = "value"))]
    SessionCurrentStart(Timestamp),
    #[strum(serialize = "Session ForcingNewSession", props(r#type = "value"))]
    SessionForcingNewSession(bool),
    #[strum(serialize = "Session NextKeyFor", props(r#type = "map"))]
    SessionNextKeyFor(AccountId, SessionKey),
    // ============================================================================================
    // ChainX
    // ============================================================================================
    // xsystem ------------------------------------------------------------------------------------
    #[strum(serialize = "XSystem BlockProducer", props(r#type = "value"))]
    XSystemBlockProducer(AccountId),
    #[strum(serialize = "XSystem NetworkProps", props(r#type = "value"))]
    XSystemNetworkProps((NetworkType, AddressType)),
    // xaccounts ----------------------------------------------------------------------------------
    #[strum(serialize = "XAccounts IntentionOf", props(r#type = "map"))]
    XAccountsIntentionOf(Name, AccountId),
    #[strum(serialize = "XAccounts IntentionNameOf", props(r#type = "map"))]
    XAccountsIntentionNameOf(AccountId, Name),
    #[strum(serialize = "XAccounts IntentionPropertiesOf", props(r#type = "map"))]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps<SessionKey>),
    #[strum(serialize = "XAccounts TeamAddress", props(r#type = "value"))]
    XAccountsTeamAddress(AccountId),
    #[strum(serialize = "XAccounts CouncilAddress", props(r#type = "value"))]
    XAccountsCouncilAddress(AccountId),
    // xfee ---------------------------------------------------------------------------------------
    #[strum(serialize = "XFeeManager Switch", props(r#type = "value"))]
    XFeeManagerSwitch(SwitchStore),
    #[strum(serialize = "XFeeManager ProducerFeeProportion", props(r#type = "value"))]
    XFeeManagerProducerFeeProportion((u32, u32)),
    #[strum(serialize = "XFeeManager TransactionBaseFee", props(r#type = "value"))]
    XFeeManagerTransactionBaseFee(Balance),
    #[strum(serialize = "XFeeManager TransactionByteFee", props(r#type = "value"))]
    XFeeManagerTransactionByteFee(Balance),
    // xassets ------------------------------------------------------------------------------------
    // XAssets
    #[strum(serialize = "XAssets AssetList", props(r#type = "map"))]
    XAssetsAssetList(Chain, Vec<Token>),
    #[strum(serialize = "XAssets AssetInfo", props(r#type = "map"))]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
    #[strum(serialize = "XAssets AssetBalance", props(r#type = "map"))]
    XAssetsAssetBalance((AccountId, Token), BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets TotalAssetBalance", props(r#type = "map"))]
    XAssetsTotalAssetBalance(Token, BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets MemoLen", props(r#type = "value"))]
    XAssetsMemoLen(u32),
    // XAssetsRecords
    #[strum(serialize = "XAssetsRecords ApplicationMHeader", props(r#type = "map"))]
    XAssetsRecordsApplicationMHeader(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMTail", props(r#type = "map"))]
    XAssetsRecordsApplicationMTail(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMap", props(r#type = "map"))]
    XAssetsRecordsApplicationMap(u32, Node<Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords SerialNumber", props(r#type = "value"))]
    XAssetsRecordsSerialNumber(u32),
    // xmining ------------------------------------------------------------------------------------
    // XStaking
    #[strum(serialize = "XStaking InitialReward", props(r#type = "value"))]
    XStakingInitialReward(Balance),
    #[strum(serialize = "XStaking ValidatorCount", props(r#type = "value"))]
    XStakingValidatorCount(u32),
    #[strum(serialize = "XStaking MinimumValidatorCount", props(r#type = "value"))]
    XStakingMinimumValidatorCount(u32),
    #[strum(serialize = "XStaking SessionsPerEra", props(r#type = "value"))]
    XStakingSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking BondingDuration", props(r#type = "value"))]
    XStakingBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking IntentionBondingDuration", props(r#type = "value"))]
    XStakingIntentionBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking SessionsPerEpoch", props(r#type = "value"))]
    XStakingSessionsPerEpoch(BlockNumber),
    #[strum(serialize = "XStaking ValidatorStakeThreshold", props(r#type = "value"))]
    XStakingValidatorStakeThreshold(Balance),
    #[strum(serialize = "XStaking CurrentEra", props(r#type = "value"))]
    XStakingCurrentEra(BlockNumber),
    #[strum(serialize = "XStaking Intentions", props(r#type = "value"))]
    XStakingIntentions(Vec<AccountId>),
    #[strum(serialize = "XStaking NextSessionsPerEra", props(r#type = "value"))]
    XStakingNextSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking LastEraLengthChange", props(r#type = "value"))]
    XStakingLastEraLengthChange(BlockNumber),
    #[strum(serialize = "XStaking ForcingNewEra", props(r#type = "value"))]
    XStakingForcingNewEra(()),
    #[strum(serialize = "XStaking StakeWeight", props(r#type = "map"))]
    XStakingStakeWeight(AccountId, Balance),
    #[strum(serialize = "XStaking IntentionProfiles", props(r#type = "map"))]
    XStakingIntentionProfiles(AccountId, IntentionProfs<Balance, BlockNumber>),
    #[strum(serialize = "XStaking NominationRecords", props(r#type = "map"))]
    XStakingNominationRecords((AccountId, AccountId), NominationRecord<Balance, BlockNumber>),
    #[strum(serialize = "XStaking MinimumPenalty", props(r#type = "value"))]
    XStakingMinimumPenalty(Balance),
    #[strum(serialize = "XStaking OfflineValidatorsPerSession", props(r#type = "value"))]
    XStakingOfflineValidatorsPerSession(Vec<AccountId>),
    #[strum(serialize = "XStaking MissedOfPerSession", props(r#type = "map"))]
    XStakingMissedOfPerSession(AccountId, u32),
    // XTokens
    #[strum(serialize = "XTokens TokenDiscount", props(r#type = "map"))]
    XTokensTokenDiscount(Token, u32),
    #[strum(serialize = "XTokens PseduIntentions", props(r#type = "value"))]
    XTokensPseduIntentions(Vec<Token>),
    #[strum(serialize = "XTokens PseduIntentionProfiles", props(r#type = "map"))]
    XTokensPseduIntentionProfiles(Token, PseduIntentionVoteWeight<BlockNumber>),
    #[strum(serialize = "XTokens DepositRecords", props(r#type = "map"))]
    XTokensDepositRecords((AccountId, Token), DepositVoteWeight<BlockNumber>),
    // xmultisig ----------------------------------------------------------------------------------
    #[strum(serialize = "XMultiSig RootAddrList", props(r#type = "value"))]
    XMultiSigRootAddrList(Vec<AccountId>),
    #[strum(serialize = "XMultiSig MultiSigAddrInfo", props(r#type = "map"))]
    XMultiSigMultiSigAddrInfo(AccountId, AddrInfo<AccountId>),
    #[strum(serialize = "XMultiSig PendingListFor", props(r#type = "map"))]
    XMultiSigPendingListFor(AccountId, Vec<Hash>),
    #[strum(serialize = "XMultiSig MultiSigListItemFor", props(r#type = "map"))]
    XMultiSigMultiSigListItemFor((AccountId, u32), AccountId),
    #[strum(serialize = "XMultiSig MultiSigListLenFor", props(r#type = "map"))]
    XMultiSigMultiSigListLenFor(AccountId, u32),
    // xdex ---------------------------------------------------------------------------------------
    // XSpot
    #[strum(serialize = "XSpot TradingPairCount", props(r#type = "value"))]
    XSpotTradingPairCount(TradingPairIndex),
    #[strum(serialize = "XSpot TradingPairOf", props(r#type = "map"))]
    XSpotTradingPairOf(TradingPairIndex, TradingPair),
    #[strum(serialize = "XSpot TradingPairInfoOf", props(r#type = "map"))]
    XSpotTradingPairInfoOf(TradingPairIndex, (Price, Price, BlockNumber)),
    #[strum(serialize = "XSpot TradeHistoryIndexOf", props(r#type = "map"))]
    XSpotTradeHistoryIndexOf(TradingPairIndex, TradeHistoryIndex),
    #[strum(serialize = "XSpot OrderCountOf", props(r#type = "map"))]
    XSpotOrderCountOf(AccountId, OrderIndex),
    #[strum(serialize = "XSpot OrderInfoOf", props(r#type = "map"))]
    XSpotOrderInfoOf((AccountId, OrderIndex), Order<TradingPairIndex, AccountId, Balance, Price, BlockNumber>),
    #[strum(serialize = "XSpot QuotationsOf", props(r#type = "map"))]
    XSpotQuotationsOf((TradingPairIndex, Price), Vec<(AccountId, OrderIndex)>),
    #[strum(serialize = "XSpot HandicapOf", props(r#type = "map"))]
    XSpotHandicapOf(TradingPairIndex, Handicap<Price>),
    #[strum(serialize = "XSpot PriceVolatility", props(r#type = "value"))]
    XSpotPriceVolatility(u32),
    // xbridge ------------------------------------------------------------------------------------
    // BTC
    #[strum(serialize = "XBridgeOfBTC BestIndex", props(r#type = "value"))]
    XBridgeOfBTCBestIndex(H256),
    #[strum(serialize = "XBridgeOfBTC BlockHashFor", props(r#type = "map"))]
    XBridgeOfBTCBlockHashFor(u32, Vec<H256>),
    #[strum(serialize = "XBridgeOfBTC BlockHeaderFor", props(r#type = "map"))]
    XBridgeOfBTCBlockHeaderFor(H256, BlockHeaderInfo),
    #[strum(serialize = "XBridgeOfBTC TxFor", props(r#type = "map"))]
    XBridgeOfBTCTxFor(H256, TxInfo),
    #[strum(serialize = "XBridgeOfBTC InputAddrFor", props(r#type = "map"))]
    XBridgeOfBTCInputAddrFor(H256, btc::Address),
    #[strum(serialize = "XBridgeOfBTC PendingDepositMap", props(r#type = "map"))]
    XBridgeOfBTCPendingDepositMap(btc::Address, Vec<DepositCache>),
    #[strum(serialize = "XBridgeOfBTC CurrentWithdrawalProposal", props(r#type = "value"))]
    XBridgeOfBTCCurrentWithdrawalProposal(WithdrawalProposal<AccountId>),
    #[strum(serialize = "XBridgeOfBTC GenesisInfo", props(r#type = "value"))]
    XBridgeOfBTCGenesisInfo((btc::BlockHeader, u32)),
    #[strum(serialize = "XBridgeOfBTC ParamsInfo", props(r#type = "value"))]
    XBridgeOfBTCParamsInfo(Params),
    #[strum(serialize = "XBridgeOfBTC NetworkId", props(r#type = "value"))]
    XBridgeOfBTCNetworkId(u32),
    #[strum(serialize = "XBridgeOfBTC ReservedBlock", props(r#type = "value"))]
    XBridgeOfBTCReservedBlock(u32),
    #[strum(serialize = "XBridgeOfBTC ConfirmationNumber", props(r#type = "value"))]
    XBridgeOfBTCConfirmationNumber(u32),
    #[strum(serialize = "XBridgeOfBTC BtcWithdrawalFee", props(r#type = "value"))]
    XBridgeOfBTCBtcWithdrawalFee(u64),
    #[strum(serialize = "XBridgeOfBTC MaxWithdrawalCount", props(r#type = "value"))]
    XBridgeOfBTCMaxWithdrawalCount(u32),
    // SDOT
    #[strum(serialize = "XBridgeOfSDOT Claims", props(r#type = "map"))]
    XBridgeOfSDOTClaims(EthereumAddress, Balance),
    #[strum(serialize = "XBridgeOfSDOT Total", props(r#type = "value"))]
    XBridgeOfSDOTTotal(Balance),
    // Features
    #[strum(serialize = "XBridgeFeatures TrusteeMultiSigAddr", props(r#type = "map"))]
    XBridgeFeaturesTrusteeMultiSigAddr(Chain, AccountId),
    #[strum(serialize = "XBridgeFeatures TrusteeInfoConfigOf", props(r#type = "map"))]
    XBridgeFeaturesTrusteeInfoConfigOf(Chain, TrusteeInfoConfig),
    #[strum(serialize = "XBridgeFeatures TrusteeSessionInfoLen", props(r#type = "map"))]
    XBridgeFeaturesTrusteeSessionInfoLen(Chain, u32),
    #[strum(serialize = "XBridgeFeatures BitcoinTrusteeSessionInfoOf", props(r#type = "map"))]
    XBridgeFeaturesBitcoinTrusteeSessionInfoOf(u32, BitcoinTrusteeSessionInfo<AccountId>),
    #[strum(serialize = "XBridgeFeatures BitcoinTrusteeIntentionPropertiesOf", props(r#type = "map"))]
    XBridgeFeaturesBitcoinTrusteeIntentionPropertiesOf(AccountId, BitcoinTrusteeIntentionProps),
    #[strum(serialize = "XBridgeFeatures BitcoinCrossChainBinding", props(r#type = "map"))]
    XBridgeFeaturesBitcoinCrossChainBinding(AccountId, Vec<btc::Address>),
    #[strum(serialize = "XBridgeFeatures BitcoinCrossChainOf", props(r#type = "map"))]
    XBridgeFeaturesBitcoinCrossChainOf(btc::Address, (AccountId, Option<AccountId>)),
    #[strum(serialize = "XBridgeFeatures EthereumCrossChainBinding", props(r#type = "map"))]
    XBridgeFeaturesEthereumCrossChainBinding(AccountId, Vec<EthereumAddress>),
    #[strum(serialize = "XBridgeFeatures EthereumCrossChainOf", props(r#type = "map"))]
    XBridgeFeaturesEthereumCrossChainOf(EthereumAddress, (AccountId, Option<AccountId>)),
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
            debug!("Empty Value: [{:?}] may have been removed", $prefix);
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
    pub fn parse(key: &[u8], value: Vec<u8>) -> Result<(&'static str, serde_json::Value)> {
        for mut storage in Self::iter() {
            let prefix: &'static str = (&storage).into();
            if key.starts_with(prefix.as_bytes()) {
                let json = storage.decode_by_type(&prefix, key, value)?;
                return Ok((prefix, json));
            }
        }
        debug!("Runtime storage parse: No matching key found");
        Err("No matching key found".into())
    }

    fn match_key<'a>(&self, prefix: &str, key: &'a [u8]) -> Result<&'a [u8]> {
        let key = match self.get_str("type") {
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
    #[allow(clippy::cyclomatic_complexity)]
    fn decode_by_type(&mut self, prefix: &str, key: &[u8], value: Vec<u8>) -> Result<serde_json::Value> {
        use RuntimeStorage::*;
        let mut key = self.match_key(prefix, key)?;

        match self {
            // Substrate ==========================================================================
            SystemAccountNonce(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            SystemBlockHash(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            IndicesNextEnumSet(ref mut v) => to_value_json!(prefix, value => v),
            IndicesEnumSet(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            TimestampNow(ref mut v) => to_value_json!(prefix, value => v),
            TimestampBlockPeriod(ref mut v) => to_value_json!(prefix, value => v),
            TimestampMinimumPeriod(ref mut v) => to_value_json!(prefix, value => v),
            TimestampWindowSize(ref mut v) => to_value_json!(prefix, value => v),
            TimestampReportLatency(ref mut v) => to_value_json!(prefix, value => v),
            SessionValidators(ref mut v) => to_value_json!(prefix, value => v),
            SessionSessionLength(ref mut v) => to_value_json!(prefix, value => v),
            SessionCurrentIndex(ref mut v) => to_value_json!(prefix, value => v),
            SessionCurrentStart(ref mut v) => to_value_json!(prefix, value => v),
            SessionForcingNewSession(ref mut v) => to_value_json!(prefix, value => v),
            SessionNextKeyFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            // ChainX =============================================================================
            // xsystem
            XSystemBlockProducer(ref mut v) => to_value_json!(prefix, value => v),
            XSystemNetworkProps(ref mut v) => to_value_json!(prefix, value => v),
            // xaccounts
            XAccountsIntentionOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAccountsIntentionNameOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAccountsIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAccountsTeamAddress(ref mut v) => to_value_json!(prefix, value => v),
            XAccountsCouncilAddress(ref mut v) => to_value_json!(prefix, value => v),
            // xfee/manager
            XFeeManagerSwitch(ref mut v) => to_value_json!(prefix, value => v),
            XFeeManagerProducerFeeProportion(ref mut v) => to_value_json!(prefix, value => v),
            XFeeManagerTransactionBaseFee(ref mut v) => to_value_json!(prefix, value => v),
            XFeeManagerTransactionByteFee(ref mut v) => to_value_json!(prefix, value => v),
            // xassets/assets
            XAssetsAssetList(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsAssetInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsTotalAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsMemoLen(ref mut v) => to_value_json!(prefix, value => v),
            // xassets/records
            XAssetsRecordsApplicationMHeader(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsRecordsApplicationMTail(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsRecordsApplicationMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XAssetsRecordsSerialNumber(ref mut v) => to_value_json!(prefix, value => v),
            // xmining/staking
            XStakingInitialReward(ref mut v) => to_value_json!(prefix, value => v),
            XStakingValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            XStakingMinimumValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            XStakingSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            XStakingBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            XStakingIntentionBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            XStakingSessionsPerEpoch(ref mut v) => to_value_json!(prefix, value => v),
            XStakingValidatorStakeThreshold(ref mut v) => to_value_json!(prefix, value => v),
            XStakingCurrentEra(ref mut v) => to_value_json!(prefix, value => v),
            XStakingIntentions(ref mut v) => to_value_json!(prefix, value => v),
            XStakingNextSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            XStakingLastEraLengthChange(ref mut v) => to_value_json!(prefix, value => v),
            XStakingForcingNewEra(ref mut v) => to_value_json!(prefix, value => v),
            XStakingStakeWeight(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XStakingIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XStakingNominationRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XStakingMinimumPenalty(ref mut v) => to_value_json!(prefix, value => v),
            XStakingOfflineValidatorsPerSession(ref mut v) => to_value_json!(prefix, value => v),
            XStakingMissedOfPerSession(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            // // xmining/tokens
            XTokensTokenDiscount(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XTokensPseduIntentions(ref mut v) => to_value_json!(prefix, value => v),
            XTokensPseduIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XTokensDepositRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            // xmultisig
            XMultiSigRootAddrList(ref mut v) => to_value_json!(prefix, value => v),
            XMultiSigMultiSigAddrInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XMultiSigPendingListFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XMultiSigMultiSigListItemFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XMultiSigMultiSigListLenFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            // xdex/spot
            XSpotTradingPairCount(ref mut v) => to_value_json!(prefix, value => v),
            XSpotTradingPairOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotTradingPairInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotTradeHistoryIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotOrderCountOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotOrderInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotQuotationsOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotHandicapOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XSpotPriceVolatility(ref mut v) => to_value_json!(prefix, value => v),
            // xbridge/bitcoin
            XBridgeOfBTCBestIndex(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCBlockHashFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfBTCTxFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfBTCInputAddrFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfBTCPendingDepositMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfBTCCurrentWithdrawalProposal(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCGenesisInfo(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCParamsInfo(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCNetworkId(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCReservedBlock(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCConfirmationNumber(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCBtcWithdrawalFee(ref mut v) => to_value_json!(prefix, value => v),
            XBridgeOfBTCMaxWithdrawalCount(ref mut v) => to_value_json!(prefix, value => v),
            // xbridge/sdot
            XBridgeOfSDOTClaims(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeOfSDOTTotal(ref mut v) => to_value_json!(prefix, value => v),
            // xbridge/features
            XBridgeFeaturesTrusteeMultiSigAddr(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesTrusteeInfoConfigOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesTrusteeSessionInfoLen(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinTrusteeSessionInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinTrusteeIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinCrossChainBinding(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinCrossChainOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesEthereumCrossChainBinding(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            XBridgeFeaturesEthereumCrossChainOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_match_value() {
        let key = "XTokens PseduIntentions".as_bytes();
        let value = hex::decode("080c4254431053444f54").unwrap();
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"value",
                "prefix":"XTokens PseduIntentions",
                "key":null,
                "value":["BTC","SDOT"]
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    fn test_parse_match_map() {
        let key = "XAssets AssetList\x00".as_bytes();
        let value = hex::decode("040c504358").unwrap();
        let (_, got) = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"map",
                "prefix":"XAssets AssetList",
                "key":"ChainX",
                "value":["PCX"]
            }"#,
        )
        .unwrap();
        assert_eq!(got, exp);
    }

    #[test]
    fn test_parse_match_map_option() {
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
    fn test_parse_match_btree_map() {
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
    fn test_parse_remove_value() {
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

    #[test]
    fn test_parse_btc_block_header_for() {
        let key: Vec<u8> = vec![
            88, 66, 114, 105, 100, 103, 101, 79, 102, 66, 84, 67, 32, 66, 108, 111, 99, 107, 72,
            101, 97, 100, 101, 114, 70, 111, 114, 17, 236, 67, 232, 134, 149, 88, 40, 181, 65, 17,
            172, 232, 106, 54, 152, 241, 119, 229, 70, 94, 82, 120, 156, 200, 250, 63, 0, 0, 0, 0,
            0,
        ];
        let value: Vec<u8> = vec![
            65, 1, 0, 0, 0, 32, 191, 83, 119, 194, 61, 87, 214, 213, 139, 39, 29, 18, 205, 101, 29,
            83, 9, 195, 158, 83, 121, 181, 78, 71, 27, 115, 48, 0, 0, 0, 0, 0, 219, 155, 212, 181,
            234, 26, 130, 11, 1, 93, 226, 194, 250, 71, 254, 219, 120, 195, 110, 151, 175, 123,
            188, 204, 169, 122, 189, 43, 13, 4, 106, 3, 113, 81, 106, 92, 4, 252, 0, 29, 201, 117,
            178, 119, 31, 64, 22, 0, 0, 0,
        ];
        let (_, got) = RuntimeStorage::parse(&key, value).unwrap();
        println!("{:?}", got);
    }
}
