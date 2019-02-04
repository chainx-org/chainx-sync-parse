mod btc;
mod btree_map;
mod linked_node;
mod primitives;

use log::{debug, error, info};
use parity_codec::Decode;
use serde_json::json;
use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{EnumIter, EnumMessage};

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
    #[strum(message = "XAccounts IntentionOf", detailed_message = "map")]
    XAccountsIntentionOf(Name, AccountId),
    #[strum(message = "XAccounts IntentionNameOf", detailed_message = "map")]
    XAccountsIntentionNameOf(AccountId, Name),
    #[strum(message = "XAccounts IntentionPropertiesOf", detailed_message = "map")]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps),
    #[strum(message = "XAccounts TrusteeIntentions", detailed_message = "value")]
    XAccountsTrusteeIntentions(Vec<AccountId>),
    #[strum(message = "XAccounts TrusteeIntentionPropertiesOf", detailed_message = "map")]
    XAccountsTrusteeIntentionPropertiesOf((AccountId, Chain), TrusteeIntentionProps),
    #[strum(message = "XAccounts CrossChainAddressMapOf", detailed_message = "map")]
    XAccountsCrossChainAddressMapOf((Chain, Vec<u8>), (AccountId, AccountId)),
    #[strum(message = "XAccounts CrossChainBindOf", detailed_message = "map")]
    XAccountsCrossChainBindOf((Chain, AccountId), Vec<Vec<u8>>),
    #[strum(message = "XAccounts TrusteeAddress", detailed_message = "map")]
    XAccountsTrusteeAddress(Chain, TrusteeAddressPair),
    // xfee ---------------------------------------------------------------------------------------
    #[strum(message = "XFeeManager Switch", detailed_message = "value")]
    XFeeManagerSwitch(bool),
    #[strum(message = "XFeeManager ProducerFeeProportion", detailed_message = "value")]
    XFeeManagerProducerFeeProportion((u32, u32)),
    // xassets ------------------------------------------------------------------------------------
    // XAssets
    #[strum(message = "XAssets AssetList", detailed_message = "map")]
    XAssetsAssetList(Chain, Vec<Token>),
    #[strum(message = "XAssets AssetInfo", detailed_message = "map")]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
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
    // BTC
    #[strum(message = "XBridgeOfBTC BestIndex", detailed_message = "value")]
    XBridgeOfBTCBestIndex(H256),
    #[strum(message = "XBridgeOfBTC BlockHeaderFor", detailed_message = "map")]
    XBridgeOfBTCBlockHeaderFor(H256, BlockHeaderInfo),
    #[strum(message = "XBridgeOfBTC BlockHeightFor", detailed_message = "map")]
    XBridgeOfBTCBlockHeightFor(u32, Vec<H256>),
    #[strum(message = "XBridgeOfBTC TxFor", detailed_message = "map")]
    XBridgeOfBTCTxFor(H256, TxInfo),
    #[strum(message = "XBridgeOfBTC GenesisInfo", detailed_message = "value")]
    XBridgeOfBTCGenesisInfo((BlockHeader, u32)),
    #[strum(message = "XBridgeOfBTC ParamsInfo", detailed_message = "value")]
    XBridgeOfBTCParamsInfo(Params),
    #[strum(message = "XBridgeOfBTC NetworkId", detailed_message = "value")]
    XBridgeOfBTCNetworkId(u32),
    #[strum(message = "XBridgeOfBTC ReservedBlock", detailed_message = "value")]
    XBridgeOfBTCReservedBlock(u32),
    #[strum(message = "XBridgeOfBTC IrrBlock", detailed_message = "value")]
    XBridgeOfBTCIrrBlock(u32),
    #[strum(message = "XBridgeOfBTC BtcFee", detailed_message = "value")]
    XBridgeOfBTCBtcFee(u64),
    #[strum(message = "XBridgeOfBTC MaxWithdrawAmount", detailed_message = "value")]
    XBridgeOfBTCMaxWithdrawAmount(u32),
    #[strum(message = "XBridgeOfBTC TxProposal", detailed_message = "value")]
    XBridgeOfBTCTxProposal(CandidateTx),
    #[strum(message = "XBridgeOfBTC PendingDepositMap", detailed_message = "map")]
    XBridgeOfBTCPendingDepositMap(btc::Address, Vec<DepositCache>),
    #[strum(message = "XBridgeOfBTC VoteNode", detailed_message = "value")]
    XBridgeOfBTCVoteNode(Vec<VoteStatus<AccountId>>),
    // XDOT
    #[strum(message = "XBridgeXDOT Claims", detailed_message = "map")]
    XBridgeXDOTClaims(EthereumAddress, Balance),
    #[strum(message = "XBridgeXdot Total", detailed_message = "value")]
    XBridgeXDOTTotal(Vec<(EthereumAddress, Balance)>),
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
            RuntimeStorage::XAccountsIntentionOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionNameOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsTrusteeIntentionPropertiesOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCrossChainAddressMapOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCrossChainBindOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsTrusteeAddress(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XFeeManagerSwitch(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XFeeManagerProducerFeeProportion(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsAssetList(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
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
            RuntimeStorage::XBridgeOfBTCBestIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCBlockHeightFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCTxFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCGenesisInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCParamsInfo(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCNetworkId(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCReservedBlock(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCIrrBlock(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCBtcFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCMaxWithdrawAmount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCTxProposal(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XBridgeOfBTCPendingDepositMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeOfBTCVoteNode(ref mut v) => to_value_json!(prefix, value => v),
            // bridge - xdot
            RuntimeStorage::XBridgeXDOTClaims(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XBridgeXDOTTotal(ref mut v) => to_value_json!(prefix, value => v),
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
