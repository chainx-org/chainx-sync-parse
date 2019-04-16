mod primitives;

use std::collections::BTreeMap;

use parity_codec::Decode;
use serde_json::json;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty, IntoStaticStr};

use self::primitives::*;
use crate::types::{btc, Bytes, MultiNodeIndex, Node};
use crate::Result;

#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq, Debug, IntoStaticStr, EnumIter, EnumProperty)]
pub enum RuntimeStorage {
    // ============================================================================================
    // Substrate
    // ============================================================================================
    // system -------------------------------------------------------------------------------------
//    #[strum(serialize = "System Number", props(type = "value"))]
//    SystemNumber(BlockNumber),
    #[strum(serialize = "System AccountNonce", props(type = "map"))]
    SystemAccountNonce(AccountId, Index),
    #[strum(serialize = "System BlockHash", props(type = "map"))]
    SystemBlockHash(BlockNumber, Hash),
    // balances -----------------------------------------------------------------------------------
    #[strum(serialize = "Balances TotalIssuance", props(type = "value"))]
    BalancesTotalIssuance(Balance),
    #[strum(serialize = "Balances ExistentialDeposit", props(type = "value"))]
    BalancesExistentialDeposit(Balance),
    #[strum(serialize = "Balances TransferFee", props(type = "value"))]
    BalancesTransferFee(Balance),
    #[strum(serialize = "Balances CreationFee", props(type = "value"))]
    BalancesCreationFee(Balance),
    #[strum(serialize = "Balances TransactionBaseFee", props(type = "value"))]
    BalancesTransactionBaseFee(Balance),
    #[strum(serialize = "Balances TransactionByteFee", props(type = "value"))]
    BalancesTransactionByteFee(Balance),
    #[strum(serialize = "Balances Vesting", props(type = "map"))]
    BalancesVesting(AccountId, VestingSchedule<Balance>),
    #[strum(serialize = "Balances FreeBalance", props(type = "map"))]
    BalancesFreeBalance(AccountId, Balance),
    #[strum(serialize = "Balances ReservedBalance", props(type = "map"))]
    BalancesReservedBalance(AccountId, Balance),
    //    #[strum(serialize = "Balances Locks", props(type = "map"))]
//    BalancesLocks(AccountId, Vec<BalanceLock<Balance, BlockNumber>>),
    // indices ------------------------------------------------------------------------------------
    #[strum(serialize = "Indices NextEnumSet", props(type = "value"))]
    IndicesNextEnumSet(AccountIndex),
    #[strum(serialize = "Indices EnumSet", props(type = "map"))]
    IndicesEnumSet(AccountIndex, Vec<AccountId>),
    // timestamp ----------------------------------------------------------------------------------
    #[strum(serialize = "Timestamp Now", props(type = "value"))]
    TimestampNow(Timestamp),
    #[strum(serialize = "Timestamp BlockPeriod", props(type = "value"))]
    TimestampBlockPeriod(Timestamp),
    // session ------------------------------------------------------------------------------------
    #[strum(serialize = "Session Validators", props(type = "value"))]
    SessionValidators(Vec<(AccountId, u64)>),
    // SessionValidators(Vec<AccountId>) in substrate
    #[strum(serialize = "Session SessionLength", props(type = "value"))]
    SessionSessionLength(BlockNumber),
    #[strum(serialize = "Session CurrentIndex", props(type = "value"))]
    SessionCurrentIndex(BlockNumber),
    #[strum(serialize = "Session CurrentStart", props(type = "value"))]
    SessionCurrentStart(Timestamp),
    #[strum(serialize = "Session ForcingNewSession", props(type = "value"))]
    SessionForcingNewSession(bool),
    #[strum(serialize = "Session NextKeyFor", props(type = "map"))]
    SessionNextKeyFor(AccountId, SessionKey),
    // ============================================================================================
    // ChainX
    // ============================================================================================
    // xsystem ------------------------------------------------------------------------------------
    #[strum(serialize = "XSystem BlockProducer", props(type = "value"))]
    XSystemBlockProducer(AccountId),
    #[strum(serialize = "XSystem DeathAccount", props(type = "value"))]
    XSystemDeathAccount(AccountId),
    #[strum(serialize = "XSystem BurnAccount", props(type = "value"))]
    XSystemBurnAccount(AccountId),
    // xaccounts ----------------------------------------------------------------------------------
    #[strum(serialize = "XAccounts IntentionOf", props(type = "map"))]
    XAccountsIntentionOf(Name, AccountId),
    #[strum(serialize = "XAccounts IntentionNameOf", props(type = "map"))]
    XAccountsIntentionNameOf(AccountId, Name),
    #[strum(serialize = "XAccounts IntentionPropertiesOf", props(type = "map"))]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps<SessionKey>),
    #[strum(serialize = "XAccounts CrossChainAddressMapOf", props(type = "map"))]
    XAccountsCrossChainAddressMapOf((Chain, Bytes), (AccountId, Option<AccountId>)),
    #[strum(serialize = "XAccounts CrossChainBindOf", props(type = "map"))]
    XAccountsCrossChainBindOf((Chain, AccountId), Vec<Bytes>),
    #[strum(serialize = "XAccounts TrusteeSessionInfoLen", props(type = "map"))]
    XAccountsTrusteeSessionInfoLen(Chain, u32),
    #[strum(serialize = "XAccounts TrusteeSessionInfoOf", props(type = "map"))]
    XAccountsTrusteeSessionInfoOf((Chain, u32), TrusteeSessionInfo<AccountId>),
    #[strum(serialize = "XAccounts TrusteeInfoConfigOf", props(type = "map"))]
    XAccountsTrusteeInfoConfigOf(Chain, TrusteeInfoConfig),
    #[strum(serialize = "XAccounts TrusteeIntentionPropertiesOf", props(type = "map"))]
    XAccountsTrusteeIntentionPropertiesOf((AccountId, Chain), TrusteeIntentionProps),
    // xfee ---------------------------------------------------------------------------------------
    #[strum(serialize = "XFeeManager Switch", props(type = "value"))]
    XFeeManagerSwitch(SwitchStore),
    #[strum(serialize = "XFeeManager ProducerFeeProportion", props(type = "value"))]
    XFeeManagerProducerFeeProportion((u32, u32)),
    #[strum(serialize = "XFeeManager TransactionBaseFee", props(type = "value"))]
    XFeeManagerTransactionBaseFee(Balance),
    #[strum(serialize = "XFeeManager TransactionByteFee", props(type = "value"))]
    XFeeManagerTransactionByteFee(Balance),
    // xassets ------------------------------------------------------------------------------------
    // XAssets
    #[strum(serialize = "XAssets AssetList", props(type = "map"))]
    XAssetsAssetList(Chain, Vec<Token>),
    #[strum(serialize = "XAssets AssetInfo", props(type = "map"))]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
    #[strum(serialize = "XAssets AssetBalance", props(type = "map"))]
    XAssetsAssetBalance((AccountId, Token), BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets TotalAssetBalance", props(type = "map"))]
    XAssetsTotalAssetBalance(Token, BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets MemoLen", props(type = "value"))]
    XAssetsMemoLen(u32),
    // XAssetsRecords
    #[strum(serialize = "XAssetsRecords ApplicationMHeader", props(type = "map"))]
    XAssetsRecordsApplicationMHeader(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMTail", props(type = "map"))]
    XAssetsRecordsApplicationMTail(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMap", props(type = "map"))]
    XAssetsRecordsApplicationMap(u32, Node<Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords SerialNumber", props(type = "value"))]
    XAssetsRecordsSerialNumber(u32),
    // xmining ------------------------------------------------------------------------------------
    // XStaking
    #[strum(serialize = "XStaking InitialReward", props(type = "value"))]
    XStakingInitialReward(Balance),
    #[strum(serialize = "XStaking ValidatorCount", props(type = "value"))]
    XStakingValidatorCount(u32),
    #[strum(serialize = "XStaking MinimumValidatorCount", props(type = "value"))]
    XStakingMinimumValidatorCount(u32),
    #[strum(serialize = "XStaking SessionsPerEra", props(type = "value"))]
    XStakingSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking BondingDuration", props(type = "value"))]
    XStakingBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking IntentionBondingDuration", props(type = "value"))]
    XStakingIntentionBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking SessionsPerEpoch", props(type = "value"))]
    XStakingSessionsPerEpoch(BlockNumber),
    #[strum(serialize = "XStaking ValidatorStakeThreshold", props(type = "value"))]
    XStakingValidatorStakeThreshold(Balance),
    #[strum(serialize = "XStaking CurrentEra", props(type = "value"))]
    XStakingCurrentEra(BlockNumber),
    #[strum(serialize = "XStaking Intentions", props(type = "value"))]
    XStakingIntentions(Vec<AccountId>),
    #[strum(serialize = "XStaking NextSessionsPerEra", props(type = "value"))]
    XStakingNextSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking LastEraLengthChange", props(type = "value"))]
    XStakingLastEraLengthChange(BlockNumber),
    #[strum(serialize = "XStaking ForcingNewEra", props(type = "value"))]
    XStakingForcingNewEra(()),
    #[strum(serialize = "XStaking StakeWeight", props(type = "map"))]
    XStakingStakeWeight(AccountId, Balance),
    #[strum(serialize = "XStaking IntentionProfiles", props(type = "map"))]
    XStakingIntentionProfiles(AccountId, IntentionProfs<Balance, BlockNumber>),
    #[strum(serialize = "XStaking NominationRecords", props(type = "map"))]
    XStakingNominationRecords((AccountId, AccountId), NominationRecord<Balance, BlockNumber>),
    #[strum(serialize = "XStaking TeamAddress", props(type = "value"))]
    XStakingTeamAddress(AccountId),
    #[strum(serialize = "XStaking CouncilAddress", props(type = "value"))]
    XStakingCouncilAddress(AccountId),
    #[strum(serialize = "XStaking MinimumPenalty", props(type = "value"))]
    XStakingMinimumPenalty(Balance),
    #[strum(serialize = "XStaking OfflineValidatorsPerSession", props(type = "value"))]
    XStakingOfflineValidatorsPerSession(Vec<AccountId>),
    #[strum(serialize = "XStaking MissedOfPerSession", props(type = "map"))]
    XStakingMissedOfPerSession(AccountId, u32),
    // XTokens
    #[strum(serialize = "XTokens TokenDiscount", props(type = "map"))]
    XTokensTokenDiscount(Token, u32),
    #[strum(serialize = "XTokens PseduIntentions", props(type = "value"))]
    XTokensPseduIntentions(Vec<Token>),
    #[strum(serialize = "XTokens PseduIntentionProfiles", props(type = "map"))]
    XTokensPseduIntentionProfiles(Token, PseduIntentionVoteWeight<BlockNumber>),
    #[strum(serialize = "XTokens DepositRecords", props(type = "map"))]
    XTokensDepositRecords((AccountId, Token), DepositVoteWeight<BlockNumber>),
    // xmultisig ----------------------------------------------------------------------------------
    #[strum(serialize = "XMultiSig RootAddrList", props(type = "value"))]
    XMultiSigRootAddrList(Vec<AccountId>),
    #[strum(serialize = "XMultiSig MultiSigAddrInfo", props(type = "map"))]
    XMultiSigMultiSigAddrInfo(AccountId, AddrInfo<AccountId>),
    #[strum(serialize = "XMultiSig PendingListFor", props(type = "map"))]
    XMultiSigPendingListFor(AccountId, Vec<Hash>),
    //    #[strum(serialize = "XMultiSig PendingStateFor", props(type = "map"))]
//    XMultiSigPendingStateFor((AccountId, Hash), PendingState<Proposal>),
    #[strum(serialize = "XMultiSig MultiSigListItemFor", props(type = "map"))]
    XMultiSigMultiSigListItemFor((AccountId, u32), AccountId),
    #[strum(serialize = "XMultiSig MultiSigListLenFor", props(type = "map"))]
    XMultiSigMultiSigListLenFor(AccountId, u32),
    #[strum(serialize = "XMultiSig TrusteeMultiSigAddr", props(type = "map"))]
    XMultiSigTrusteeMultiSigAddr(Chain, AccountId),
    // xdex ---------------------------------------------------------------------------------------
    // XSpot
    #[strum(serialize = "XSpot TradingPairCount", props(type = "value"))]
    XSpotTradingPairCount(TradingPairIndex),
    #[strum(serialize = "XSpot TradingPairOf", props(type = "map"))]
    XSpotTradingPairOf(TradingPairIndex, TradingPair),
    #[strum(serialize = "XSpot TradingPairInfoOf", props(type = "map"))]
    XSpotTradingPairInfoOf(TradingPairIndex, (Price, Price, BlockNumber)),
    #[strum(serialize = "XSpot TradeHistoryIndexOf", props(type = "map"))]
    XSpotTradeHistoryIndexOf(TradingPairIndex, TradeHistoryIndex),
    #[strum(serialize = "XSpot OrderCountOf", props(type = "map"))]
    XSpotOrderCountOf(AccountId, OrderIndex),
    #[strum(serialize = "XSpot OrderInfoOf", props(type = "map"))]
    XSpotOrderInfoOf((AccountId, OrderIndex), Order<TradingPairIndex, AccountId, Amount, Price, BlockNumber>),
    #[strum(serialize = "XSpot QuotationsOf", props(type = "map"))]
    XSpotQuotationsOf((TradingPairIndex, Price), Vec<(AccountId, OrderIndex)>),
    #[strum(serialize = "XSpot HandicapOf", props(type = "map"))]
    XSpotHandicapOf(TradingPairIndex, Handicap<Price>),
    #[strum(serialize = "XSpot PriceVolatility", props(type = "value"))]
    XSpotPriceVolatility(u32),
    // xbridge ------------------------------------------------------------------------------------
    // BTC
    #[strum(serialize = "XBridgeOfBTC BestIndex", props(type = "value"))]
    XBridgeOfBTCBestIndex(H256),
    #[strum(serialize = "XBridgeOfBTC BlockHashFor", props(type = "map"))]
    XBridgeOfBTCBlockHashFor(u32, Vec<H256>),
    #[strum(serialize = "XBridgeOfBTC BlockHeaderFor", props(type = "map"))]
    XBridgeOfBTCBlockHeaderFor(H256, BlockHeaderInfo),
    #[strum(serialize = "XBridgeOfBTC TxFor", props(type = "map"))]
    XBridgeOfBTCTxFor(H256, TxInfo),
    #[strum(serialize = "XBridgeOfBTC InputAddrFor", props(type = "map"))]
    XBridgeOfBTCInputAddrFor(H256, btc::Address),
    #[strum(serialize = "XBridgeOfBTC PendingDepositMap", props(type = "map"))]
    XBridgeOfBTCPendingDepositMap(btc::Address, Vec<DepositCache>),
    #[strum(serialize = "XBridgeOfBTC CurrentWithdrawalProposal", props(type = "value"))]
    XBridgeOfBTCCurrentWithdrawalProposal(WithdrawalProposal<AccountId>),
    #[strum(serialize = "XBridgeOfBTC GenesisInfo", props(type = "value"))]
    XBridgeOfBTCGenesisInfo((btc::BlockHeader, u32)),
    #[strum(serialize = "XBridgeOfBTC ParamsInfo", props(type = "value"))]
    XBridgeOfBTCParamsInfo(Params),
    #[strum(serialize = "XBridgeOfBTC NetworkId", props(type = "value"))]
    XBridgeOfBTCNetworkId(u32),
    #[strum(serialize = "XBridgeOfBTC ReservedBlock", props(type = "value"))]
    XBridgeOfBTCReservedBlock(u32),
    #[strum(serialize = "XBridgeOfBTC ConfirmationNumber", props(type = "value"))]
    XBridgeOfBTCConfirmationNumber(u32),
    #[strum(serialize = "XBridgeOfBTC BtcWithdrawalFee", props(type = "value"))]
    XBridgeOfBTCBtcWithdrawalFee(u64),
    #[strum(serialize = "XBridgeOfBTC MaxWithdrawalCount", props(type = "value"))]
    XBridgeOfBTCMaxWithdrawalCount(u32),
    #[strum(serialize = "XBridgeOfBTC LastTrusteeSessionNumber", props(type = "value"))]
    XBridgeOfBTCLastTrusteeSessionNumber(u32),
    // SDOT
    #[strum(serialize = "XBridgeOfSDOT Claims", props(type = "map"))]
    XBridgeOfSDOTClaims(EthereumAddress, Balance),
    #[strum(serialize = "XBridgeOfSDOT Total", props(type = "value"))]
    XBridgeOfSDOTTotal(Balance),
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
            RuntimeStorage::BalancesTransactionBaseFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTransactionByteFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesVesting(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesFreeBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesReservedBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BalancesLocks(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
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
            RuntimeStorage::XSystemBurnAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsIntentionOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionNameOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCrossChainAddressMapOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCrossChainBindOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeSessionInfoLen(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeSessionInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeInfoConfigOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XFeeManagerSwitch(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XFeeManagerProducerFeeProportion(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XFeeManagerTransactionBaseFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XFeeManagerTransactionByteFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsAssetList(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsTotalAssetBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsMemoLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMHeader(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMTail(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsApplicationMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsSerialNumber(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingInitialReward(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingMinimumValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingIntentionBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingSessionsPerEpoch(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingValidatorStakeThreshold(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCurrentEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingNextSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingLastEraLengthChange(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingForcingNewEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingStakeWeight(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingNominationRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingTeamAddress(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCouncilAddress(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingMinimumPenalty(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingOfflineValidatorsPerSession(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingMissedOfPerSession(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensTokenDiscount(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensPseduIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XTokensPseduIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensDepositRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMultiSigRootAddrList(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMultiSigMultiSigAddrInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMultiSigPendingListFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMultiSigPendingStateFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMultiSigMultiSigListItemFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMultiSigMultiSigListLenFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMultiSigTrusteeMultiSigAddr(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotTradingPairCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSpotTradingPairOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotTradingPairInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotTradeHistoryIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotOrderCountOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotOrderInfoOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotQuotationsOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotHandicapOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XSpotPriceVolatility(ref mut v) => to_value_json!(prefix, value => v),
            // bridge - bitcoin
            RuntimeStorage::XBridgeOfBTCBestIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCBlockHashFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCTxFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCInputAddrFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCPendingDepositMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCCurrentWithdrawalProposal(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCGenesisInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCParamsInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCNetworkId(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCReservedBlock(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCConfirmationNumber(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCBtcWithdrawalFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCMaxWithdrawalCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCLastTrusteeSessionNumber(ref mut v) => to_value_json!(prefix, value => v),
            // bridge - xdot
            RuntimeStorage::XBridgeOfSDOTClaims(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfSDOTTotal(ref mut v) => to_value_json!(prefix, value => v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_match_value() {
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
    fn test_parse_match_map() {
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
    fn test_parse_match_codec_btree_map() {
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
