#[macro_use]
mod macros;
mod primitives;

use std::collections::BTreeMap;

use parity_codec::Decode;
use strum::{EnumIter, EnumProperty, IntoEnumIterator, IntoStaticStr};

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
//    #[strum(serialize = "System Number", props(Type = "value"))]
//    SystemNumber(BlockNumber),
    #[strum(serialize = "System AccountNonce", props(Type = "map"))]
    SystemAccountNonce(AccountId, Index),
    #[strum(serialize = "System BlockHash", props(Type = "map"))]
    SystemBlockHash(BlockNumber, H256),
    // indices ------------------------------------------------------------------------------------
    #[strum(serialize = "Indices NextEnumSet", props(Type = "value"))]
    IndicesNextEnumSet(AccountIndex),
    #[strum(serialize = "Indices EnumSet", props(Type = "map"))]
    IndicesEnumSet(AccountIndex, Vec<AccountId>),
    // timestamp ----------------------------------------------------------------------------------
    #[strum(serialize = "Timestamp Now", props(Type = "value"))]
    TimestampNow(Timestamp),
    #[strum(serialize = "Timestamp BlockPeriod", props(Type = "value"))]
    TimestampBlockPeriod(Timestamp),
    #[strum(serialize = "Timestamp MinimumPeriod", props(Type = "value"))]
    TimestampMinimumPeriod(Timestamp),
    // finality_tracker ---------------------------------------------------------------------------
    #[strum(serialize = "Timestamp WindowSize", props(Type = "value"))]
    TimestampWindowSize(BlockNumber),
    #[strum(serialize = "Timestamp ReportLatency", props(Type = "value"))]
    TimestampReportLatency(BlockNumber),
    // session ------------------------------------------------------------------------------------
    #[strum(serialize = "Session Validators", props(Type = "value"))]
    SessionValidators(Vec<(AccountId, u64)>),
    #[strum(serialize = "Session SessionLength", props(Type = "value"))]
    SessionSessionLength(BlockNumber),
    #[strum(serialize = "Session CurrentIndex", props(Type = "value"))]
    SessionCurrentIndex(BlockNumber),
    #[strum(serialize = "Session CurrentStart", props(Type = "value"))]
    SessionCurrentStart(Timestamp),
    #[strum(serialize = "Session SessionTotalMissedBlocksCount", props(Type = "value"))]
    SessionSessionTotalMissedBlocksCount(u32),
    #[strum(serialize = "Session ForcingNewSession", props(Type = "value"))]
    SessionForcingNewSession(bool),
    // ============================================================================================
    // ChainX
    // ============================================================================================
    // xsystem ------------------------------------------------------------------------------------
    #[strum(serialize = "XSystem BlockProducer", props(Type = "value"))]
    XSystemBlockProducer(AccountId),
    #[strum(serialize = "XSystem NetworkProps", props(Type = "value"))]
    XSystemNetworkProps((NetworkType, AddressType)),
    // xaccounts ----------------------------------------------------------------------------------
    #[strum(serialize = "XAccounts IntentionOf", props(Type = "map"))]
    XAccountsIntentionOf(Name, AccountId),
    #[strum(serialize = "XAccounts IntentionNameOf", props(Type = "map"))]
    XAccountsIntentionNameOf(AccountId, Name),
    #[strum(serialize = "XAccounts IntentionPropertiesOf", props(Type = "map"))]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps<SessionKey, BlockNumber>),
    #[strum(serialize = "XAccounts TeamAddress", props(Type = "value"))]
    XAccountsTeamAddress(AccountId),
    #[strum(serialize = "XAccounts CouncilAddress", props(Type = "value"))]
    XAccountsCouncilAddress(AccountId),
    #[strum(serialize = "XAccounts BlockedAccounts", props(Type = "value"))]
    XAccountsBlockedAccounts(Vec<AccountId>),
    // xfee ---------------------------------------------------------------------------------------
    #[strum(serialize = "XFeeManager Switcher", props(Type = "value"))]
    XFeeManagerSwitcher(BTreeMap<CallSwitcher, bool>),
    #[strum(serialize = "XFeeManager MethodCallWeight", props(Type = "value"))]
    XFeeManagerMethodCallWeight(BTreeMap<XString, u64>),
    #[strum(serialize = "XFeeManager ProducerFeeProportion", props(Type = "value"))]
    XFeeManagerProducerFeeProportion((u32, u32)),
    #[strum(serialize = "XFeeManager TransactionBaseFee", props(Type = "value"))]
    XFeeManagerTransactionBaseFee(Balance),
    #[strum(serialize = "XFeeManager TransactionByteFee", props(Type = "value"))]
    XFeeManagerTransactionByteFee(Balance),
    // xassets ------------------------------------------------------------------------------------
    // XAssets
    #[strum(serialize = "XAssets AssetList", props(Type = "map"))]
    XAssetsAssetList(Chain, Vec<Token>),
    #[strum(serialize = "XAssets AssetInfo", props(Type = "map"))]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
    #[strum(serialize = "XAssets AssetLimitProps", props(Type = "map"))]
    XAssetsAssetLimitProps(Token, BTreeMap<AssetLimit, bool>),
    #[strum(serialize = "XAssets AssetBalance", props(Type = "map"))]
    XAssetsAssetBalance((AccountId, Token), BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets TotalAssetBalance", props(Type = "map"))]
    XAssetsTotalAssetBalance(Token, BTreeMap<AssetType, Balance>),
    #[strum(serialize = "XAssets MemoLen", props(Type = "value"))]
    XAssetsMemoLen(u32),
    // XAssetsRecords
    #[strum(serialize = "XAssetsRecords ApplicationMHeader", props(Type = "map"))]
    XAssetsRecordsApplicationMHeader(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMTail", props(Type = "map"))]
    XAssetsRecordsApplicationMTail(Chain, MultiNodeIndex<Chain, Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords ApplicationMap", props(Type = "map"))]
    XAssetsRecordsApplicationMap(u32, Node<Application<AccountId, Balance, Timestamp>>),
    #[strum(serialize = "XAssetsRecords SerialNumber", props(Type = "value"))]
    XAssetsRecordsSerialNumber(u32),
    // xfisher ------------------------------------------------------------------------------------
    #[strum(serialize = "XFisher Reported", props(Type = "map"))]
    XFisherReported(H512, ()),
    #[strum(serialize = "XFisher Fishermen", props(Type = "value"))]
    XFisherFishermen(Vec<AccountId>),
    // xmining ------------------------------------------------------------------------------------
    // XStaking
    #[strum(serialize = "XStaking InitialReward", props(Type = "value"))]
    XStakingInitialReward(Balance),
    #[strum(serialize = "XStaking ValidatorCount", props(Type = "value"))]
    XStakingValidatorCount(u32),
    #[strum(serialize = "XStaking MinimumValidatorCount", props(Type = "value"))]
    XStakingMinimumValidatorCount(u32),
    #[strum(serialize = "XStaking SessionsPerEra", props(Type = "value"))]
    XStakingSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking BondingDuration", props(Type = "value"))]
    XStakingBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking IntentionBondingDuration", props(Type = "value"))]
    XStakingIntentionBondingDuration(BlockNumber),
    #[strum(serialize = "XStaking MaximumIntentionCount", props(Type = "value"))]
    XStakingMaximumIntentionCount(u32),
    #[strum(serialize = "XStaking SessionsPerEpoch", props(Type = "value"))]
    XStakingSessionsPerEpoch(BlockNumber),
    #[strum(serialize = "XStaking CurrentEra", props(Type = "value"))]
    XStakingCurrentEra(BlockNumber),
    #[strum(serialize = "XStaking DistributionRatio", props(Type = "value"))]
    XStakingDistributionRatio((u32, u32)),
    #[strum(serialize = "XStaking NextSessionsPerEra", props(Type = "value"))]
    XStakingNextSessionsPerEra(BlockNumber),
    #[strum(serialize = "XStaking LastEraLengthChange", props(Type = "value"))]
    XStakingLastEraLengthChange(BlockNumber),
    #[strum(serialize = "XStaking ForcingNewEra", props(Type = "value"))]
    XStakingForcingNewEra(()),
    #[strum(serialize = "XStaking StakeWeight", props(Type = "map"))]
    XStakingStakeWeight(AccountId, Balance),
    #[strum(serialize = "XStaking Intentions", props(Type = "linked_map"))]
    XStakingIntentions(AccountId, IntentionProfs<Balance, BlockNumber>),
    #[strum(serialize = "XStaking IntentionsV1", props(Type = "linked_map"))]
    XStakingIntentionsV1(AccountId, IntentionProfsV1<Balance, BlockNumber>),
    #[strum(serialize = "XStaking NominationRecords", props(Type = "map"))]
    XStakingNominationRecords((AccountId, AccountId), NominationRecord<Balance, BlockNumber>),
    #[strum(serialize = "XStaking NominationRecordsV1", props(Type = "map"))]
    XStakingNominationRecordsV1((AccountId, AccountId), NominationRecordV1<Balance, BlockNumber>),
    #[strum(serialize = "XStaking UpperBoundFactor", props(Type = "value"))]
    XStakingUpperBoundFactor(u32),
    #[strum(serialize = "XStaking EvilValidatorsPerSession", props(Type = "value"))]
    XStakingEvilValidatorsPerSession(Vec<AccountId>),
    #[strum(serialize = "XStaking LastRenominationOf", props(Type = "map"))]
    XStakingLastRenominationOf(AccountId, BlockNumber),
    #[strum(serialize = "XStaking MaxUnbondEntriesPerIntention", props(Type = "value"))]
    XStakingMaxUnbondEntriesPerIntention(u32),
    #[strum(serialize = "XStaking MinimumPenalty", props(Type = "value"))]
    XStakingMinimumPenalty(Balance),
    #[strum(serialize = "XStaking OfflineValidatorsPerSession", props(Type = "value"))]
    XStakingOfflineValidatorsPerSession(Vec<AccountId>),
    #[strum(serialize = "XStaking MissedOfPerSession", props(Type = "map"))]
    XStakingMissedOfPerSession(AccountId, u32),
    #[strum(serialize = "XStaking MissedBlockSeverity", props(Type = "value"))]
    XStakingMissedBlockSeverity(u32),
    // XTokens
    #[strum(serialize = "XTokens TokenDiscount", props(Type = "map"))]
    XTokensTokenDiscount(Token, u32),
    #[strum(serialize = "XTokens PseduIntentions", props(Type = "value"))]
    XTokensPseduIntentions(Vec<Token>),
    #[strum(serialize = "XTokens ClaimRestrictionOf", props(Type = "map"))]
    XTokensClaimRestrictionOf(Token, (u32, BlockNumber)),
    #[strum(serialize = "XTokens LastClaimOf", props(Type = "map"))]
    XTokensLastClaimOf((AccountId, Token), BlockNumber),
    #[strum(serialize = "XTokens PseduIntentionProfiles", props(Type = "map"))]
    XTokensPseduIntentionProfiles(Token, PseduIntentionVoteWeight<BlockNumber>),
    #[strum(serialize = "XTokens PseduIntentionProfilesV1", props(Type = "map"))]
    XTokensPseduIntentionProfilesV1(Token, PseduIntentionVoteWeightV1<BlockNumber>),
    #[strum(serialize = "XTokens DepositRecords", props(Type = "map"))]
    XTokensDepositRecords((AccountId, Token), DepositVoteWeight<BlockNumber>),
    #[strum(serialize = "XTokens DepositRecordsV1", props(Type = "map"))]
    XTokensDepositRecordsV1((AccountId, Token), DepositVoteWeightV1<BlockNumber>),
    #[strum(serialize = "XTokens DepositReward", props(Type = "value"))]
    XTokensDepositReward(Balance),
    // xmultisig ----------------------------------------------------------------------------------
    #[strum(serialize = "XMultiSig RootAddrList", props(Type = "value"))]
    XMultiSigRootAddrList(Vec<AccountId>),
    #[strum(serialize = "XMultiSig MultiSigAddrInfo", props(Type = "map"))]
    XMultiSigMultiSigAddrInfo(AccountId, AddrInfo<AccountId>),
    #[strum(serialize = "XMultiSig PendingListFor", props(Type = "map"))]
    XMultiSigPendingListFor(AccountId, Vec<H256>),
    #[strum(serialize = "XMultiSig MultiSigListItemFor", props(Type = "map"))]
    XMultiSigMultiSigListItemFor((AccountId, u32), AccountId),
    #[strum(serialize = "XMultiSig MultiSigListLenFor", props(Type = "map"))]
    XMultiSigMultiSigListLenFor(AccountId, u32),
    // xdex ---------------------------------------------------------------------------------------
    // XSpot
    #[strum(serialize = "XSpot TradingPairCount", props(Type = "value"))]
    XSpotTradingPairCount(TradingPairIndex),
    #[strum(serialize = "XSpot TradingPairOf", props(Type = "map"))]
    XSpotTradingPairOf(TradingPairIndex, TradingPair),
    #[strum(serialize = "XSpot TradingPairInfoOf", props(Type = "map"))]
    XSpotTradingPairInfoOf(TradingPairIndex, (Price, Price, BlockNumber)),
    #[strum(serialize = "XSpot TradeHistoryIndexOf", props(Type = "map"))]
    XSpotTradeHistoryIndexOf(TradingPairIndex, TradeHistoryIndex),
    #[strum(serialize = "XSpot OrderCountOf", props(Type = "map"))]
    XSpotOrderCountOf(AccountId, OrderIndex),
    #[strum(serialize = "XSpot OrderInfoOf", props(Type = "map"))]
    XSpotOrderInfoOf((AccountId, OrderIndex), Order<TradingPairIndex, AccountId, Balance, Price, BlockNumber>),
    #[strum(serialize = "XSpot QuotationsOf", props(Type = "map"))]
    XSpotQuotationsOf((TradingPairIndex, Price), Vec<(AccountId, OrderIndex)>),
    #[strum(serialize = "XSpot HandicapOf", props(Type = "map"))]
    XSpotHandicapOf(TradingPairIndex, Handicap<Price>),
    #[strum(serialize = "XSpot PriceVolatility", props(Type = "value"))]
    XSpotPriceVolatility(u32),
    // xbridge ------------------------------------------------------------------------------------
    // common
    #[strum(serialize = "XBridgeCommon CrossChainBinding", props(Type = "map"))]
    XBridgeCommonCrossChainBinding((Token, AccountId), AccountId),
    // BTC
    #[strum(serialize = "XBridgeOfBTC BestIndex", props(Type = "value"))]
    XBridgeOfBTCBestIndex(H256),
    #[strum(serialize = "XBridgeOfBTC BlockHashFor", props(Type = "map"))]
    XBridgeOfBTCBlockHashFor(u32, Vec<H256>),
    #[strum(serialize = "XBridgeOfBTC BlockHeaderFor", props(Type = "map"))]
    XBridgeOfBTCBlockHeaderFor(H256, BlockHeaderInfo),
    #[strum(serialize = "XBridgeOfBTC TxFor", props(Type = "map"))]
    XBridgeOfBTCTxFor(H256, TxInfo),
    #[strum(serialize = "XBridgeOfBTC TxMarkFor", props(Type = "map"))]
    XBridgeOfBTCTxMarkFor(H256, ()),
    #[strum(serialize = "XBridgeOfBTC InputAddrFor", props(Type = "map"))]
    XBridgeOfBTCInputAddrFor(H256, btc::Address),
    #[strum(serialize = "XBridgeOfBTC PendingDepositMap", props(Type = "map"))]
    XBridgeOfBTCPendingDepositMap(btc::Address, Vec<DepositCache>),
    #[strum(serialize = "XBridgeOfBTC CurrentWithdrawalProposal", props(Type = "value"))]
    XBridgeOfBTCCurrentWithdrawalProposal(WithdrawalProposal<AccountId>),
    #[strum(serialize = "XBridgeOfBTC GenesisInfo", props(Type = "value"))]
    XBridgeOfBTCGenesisInfo((btc::BlockHeader, u32)),
    #[strum(serialize = "XBridgeOfBTC ParamsInfo", props(Type = "value"))]
    XBridgeOfBTCParamsInfo(Params),
    #[strum(serialize = "XBridgeOfBTC NetworkId", props(Type = "value"))]
    XBridgeOfBTCNetworkId(u32),
    #[strum(serialize = "XBridgeOfBTC ReservedBlock", props(Type = "value"))]
    XBridgeOfBTCReservedBlock(u32),
    #[strum(serialize = "XBridgeOfBTC ConfirmationNumber", props(Type = "value"))]
    XBridgeOfBTCConfirmationNumber(u32),
    #[strum(serialize = "XBridgeOfBTC BtcWithdrawalFee", props(Type = "value"))]
    XBridgeOfBTCBtcWithdrawalFee(u64),
    #[strum(serialize = "XBridgeOfBTC BtcMinDeposit", props(Type = "value"))]
    XBridgeOfBTCBtcMinDeposit(u64),
    #[strum(serialize = "XBridgeOfBTC MaxWithdrawalCount", props(Type = "value"))]
    XBridgeOfBTCMaxWithdrawalCount(u32),
    // BTC lockup
    #[strum(serialize = "XBridgeOfBTCLockup LockedUpBTC", props(Type = "map"))]
    XBridgeOfBTCLockupLockedUpBTC((H256, u32), (AccountId, u64, btc::Address)),
    #[strum(serialize = "XBridgeOfBTCLockup AddressLockedCoin", props(Type = "map"))]
    XBridgeOfBTCLockupAddressLockedCoin(btc::Address, u64),
    #[strum(serialize = "XBridgeOfBTCLockup LockedCoinLimit", props(Type = "value"))]
    XBridgeOfBTCLockupLockedCoinLimit((u64, u64)),
    // SDOT
    #[strum(serialize = "XBridgeOfSDOT Claims", props(Type = "map"))]
    XBridgeOfSDOTClaims(EthereumAddress, Balance),
    #[strum(serialize = "XBridgeOfSDOT Total", props(Type = "value"))]
    XBridgeOfSDOTTotal(Balance),
    // Features
    #[strum(serialize = "XBridgeFeatures TrusteeMultiSigAddr", props(Type = "map"))]
    XBridgeFeaturesTrusteeMultiSigAddr(Chain, AccountId),
    #[strum(serialize = "XBridgeFeatures TrusteeInfoConfigOf", props(Type = "map"))]
    XBridgeFeaturesTrusteeInfoConfigOf(Chain, TrusteeInfoConfig),
    #[strum(serialize = "XBridgeFeatures TrusteeSessionInfoLen", props(Type = "map"))]
    XBridgeFeaturesTrusteeSessionInfoLen(Chain, u32),
    #[strum(serialize = "XBridgeFeatures BitcoinTrusteeSessionInfoOf", props(Type = "map"))]
    XBridgeFeaturesBitcoinTrusteeSessionInfoOf(u32, BitcoinTrusteeSessionInfo<AccountId>),
    #[strum(serialize = "XBridgeFeatures BitcoinTrusteeIntentionPropertiesOf", props(Type = "map"))]
    XBridgeFeaturesBitcoinTrusteeIntentionPropertiesOf(AccountId, BitcoinTrusteeIntentionProps),
    #[strum(serialize = "XBridgeFeatures BitcoinCrossChainBinding", props(Type = "map"))]
    XBridgeFeaturesBitcoinCrossChainBinding(AccountId, Vec<btc::Address>),
    #[strum(serialize = "XBridgeFeatures BitcoinCrossChainOf", props(Type = "map"))]
    XBridgeFeaturesBitcoinCrossChainOf(btc::Address, (AccountId, Option<AccountId>)),
    #[strum(serialize = "XBridgeFeatures EthereumCrossChainBinding", props(Type = "map"))]
    XBridgeFeaturesEthereumCrossChainBinding(AccountId, Vec<EthereumAddress>),
    #[strum(serialize = "XBridgeFeatures EthereumCrossChainOf", props(Type = "map"))]
    XBridgeFeaturesEthereumCrossChainOf(EthereumAddress, (AccountId, Option<AccountId>)),
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
        let key = match self.get_str("Type") {
            Some("map") | Some("linked_map") => &key[prefix.len()..],
            Some("value") => key,
            _ => {
                error!("Runtime storage parse: get storage type failed");
                return Err("Invalid storage type".into());
            }
        };
        Ok(key)
    }

    #[rustfmt::skip]
    #[allow(clippy::cognitive_complexity)]
    fn decode_by_type(&mut self, prefix: &str, key: &[u8], value: Vec<u8>) -> Result<serde_json::Value> {
        use RuntimeStorage::*;
        let mut key = self.match_key(prefix, key)?;

        match self {
            // Substrate ==========================================================================
            SystemAccountNonce(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            SystemBlockHash(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            IndicesNextEnumSet(ref mut v) => to_json!(prefix, value => v),
            IndicesEnumSet(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            TimestampNow(ref mut v) => to_json!(prefix, value => v),
            TimestampBlockPeriod(ref mut v) => to_json!(prefix, value => v),
            TimestampMinimumPeriod(ref mut v) => to_json!(prefix, value => v),
            TimestampWindowSize(ref mut v) => to_json!(prefix, value => v),
            TimestampReportLatency(ref mut v) => to_json!(prefix, value => v),
            SessionValidators(ref mut v) => to_json!(prefix, value => v),
            SessionSessionLength(ref mut v) => to_json!(prefix, value => v),
            SessionCurrentIndex(ref mut v) => to_json!(prefix, value => v),
            SessionCurrentStart(ref mut v) => to_json!(prefix, value => v),
            SessionSessionTotalMissedBlocksCount(ref mut v) => to_json!(prefix, value => v),
            SessionForcingNewSession(ref mut v) => to_json!(prefix, value => v),
            // ChainX =============================================================================
            // xsystem
            XSystemBlockProducer(ref mut v) => to_json!(prefix, value => v),
            XSystemNetworkProps(ref mut v) => to_json!(prefix, value => v),
            // xaccounts
            XAccountsIntentionOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAccountsIntentionNameOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAccountsIntentionPropertiesOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAccountsTeamAddress(ref mut v) => to_json!(prefix, value => v),
            XAccountsCouncilAddress(ref mut v) => to_json!(prefix, value => v),
            XAccountsBlockedAccounts(ref mut v) => to_json!(prefix, value => v),
            // xfee/manager
            XFeeManagerSwitcher(ref mut v) => to_json!(prefix, value => v),
            XFeeManagerMethodCallWeight(ref mut v) => to_json!(prefix, value => v),
            XFeeManagerProducerFeeProportion(ref mut v) => to_json!(prefix, value => v),
            XFeeManagerTransactionBaseFee(ref mut v) => to_json!(prefix, value => v),
            XFeeManagerTransactionByteFee(ref mut v) => to_json!(prefix, value => v),
            // xassets/assets
            XAssetsAssetList(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsAssetInfo(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsAssetLimitProps(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsAssetBalance(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsTotalAssetBalance(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsMemoLen(ref mut v) => to_json!(prefix, value => v),
            // xassets/records
            XAssetsRecordsApplicationMHeader(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsRecordsApplicationMTail(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsRecordsApplicationMap(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XAssetsRecordsSerialNumber(ref mut v) => to_json!(prefix, value => v),
            // xfisher
            XFisherReported(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XFisherFishermen(ref mut v) => to_json!(prefix, value => v),
            // xmining/staking
            XStakingInitialReward(ref mut v) => to_json!(prefix, value => v),
            XStakingValidatorCount(ref mut v) => to_json!(prefix, value => v),
            XStakingMinimumValidatorCount(ref mut v) => to_json!(prefix, value => v),
            XStakingSessionsPerEra(ref mut v) => to_json!(prefix, value => v),
            XStakingBondingDuration(ref mut v) => to_json!(prefix, value => v),
            XStakingIntentionBondingDuration(ref mut v) => to_json!(prefix, value => v),
            XStakingMaximumIntentionCount(ref mut v) => to_json!(prefix, value => v),
            XStakingSessionsPerEpoch(ref mut v) => to_json!(prefix, value => v),
            XStakingCurrentEra(ref mut v) => to_json!(prefix, value => v),
            XStakingDistributionRatio(ref mut v) => to_json!(prefix, value => v),
            XStakingNextSessionsPerEra(ref mut v) => to_json!(prefix, value => v),
            XStakingLastEraLengthChange(ref mut v) => to_json!(prefix, value => v),
            XStakingForcingNewEra(ref mut v) => to_json!(prefix, value => v),
            XStakingStakeWeight(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingIntentions(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingIntentionsV1(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingNominationRecords(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingNominationRecordsV1(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingUpperBoundFactor(ref mut v) => to_json!(prefix, value => v),
            XStakingEvilValidatorsPerSession(ref mut v) => to_json!(prefix, value => v),
            XStakingLastRenominationOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingMaxUnbondEntriesPerIntention(ref mut v) => to_json!(prefix, value => v),
            XStakingMinimumPenalty(ref mut v) => to_json!(prefix, value => v),
            XStakingOfflineValidatorsPerSession(ref mut v) => to_json!(prefix, value => v),
            XStakingMissedOfPerSession(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XStakingMissedBlockSeverity(ref mut v) => to_json!(prefix, value => v),
            // xmining/tokens
            XTokensTokenDiscount(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensPseduIntentions(ref mut v) => to_json!(prefix, value => v),
            XTokensClaimRestrictionOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensLastClaimOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensPseduIntentionProfiles(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensPseduIntentionProfilesV1(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensDepositRecords(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensDepositRecordsV1(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XTokensDepositReward(ref mut v) => to_json!(prefix, value => v),
            // xmultisig
            XMultiSigRootAddrList(ref mut v) => to_json!(prefix, value => v),
            XMultiSigMultiSigAddrInfo(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XMultiSigPendingListFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XMultiSigMultiSigListItemFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XMultiSigMultiSigListLenFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            // xdex/spot
            XSpotTradingPairCount(ref mut v) => to_json!(prefix, value => v),
            XSpotTradingPairOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotTradingPairInfoOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotTradeHistoryIndexOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotOrderCountOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotOrderInfoOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotQuotationsOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotHandicapOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XSpotPriceVolatility(ref mut v) => to_json!(prefix, value => v),
            // xbridge/common
            XBridgeCommonCrossChainBinding(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            // xbridge/btc
            XBridgeOfBTCBestIndex(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCBlockHashFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCTxFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCTxMarkFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCInputAddrFor(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCPendingDepositMap(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCCurrentWithdrawalProposal(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCGenesisInfo(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCParamsInfo(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCNetworkId(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCReservedBlock(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCConfirmationNumber(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCBtcWithdrawalFee(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCBtcMinDeposit(ref mut v) => to_json!(prefix, value => v),
            XBridgeOfBTCMaxWithdrawalCount(ref mut v) => to_json!(prefix, value => v),
            // xbridge/btc lockup
            XBridgeOfBTCLockupLockedUpBTC(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCLockupAddressLockedCoin(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfBTCLockupLockedCoinLimit(ref mut v) => to_json!(prefix, value => v),
            // xbridge/sdot
            XBridgeOfSDOTClaims(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeOfSDOTTotal(ref mut v) => to_json!(prefix, value => v),
            // xbridge/features
            XBridgeFeaturesTrusteeMultiSigAddr(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesTrusteeInfoConfigOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesTrusteeSessionInfoLen(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinTrusteeSessionInfoOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinTrusteeIntentionPropertiesOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinCrossChainBinding(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesBitcoinCrossChainOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesEthereumCrossChainBinding(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
            XBridgeFeaturesEthereumCrossChainOf(ref mut k, ref mut v) => to_json!(prefix, key => k, value => v),
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
