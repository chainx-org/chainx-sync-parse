mod primitives;

use std::str::FromStr;

use parity_codec::Decode;
use strum::{EnumMessage, IntoEnumIterator};

use self::primitives::*;
use error::Result;
use serde_ext::Bytes;

#[rustfmt::skip]
#[derive(EnumIter, EnumMessage, Debug, Eq, PartialEq)]
pub enum RuntimeStorage {
    // substrate
    #[strum(message = "System AccountNonce", detailed_message = "map")]
    SystemAccountNonce(AccountId, Index),
    #[strum(message = "System BlockHash", detailed_message = "map")]
    SystemBlockHash(BlockNumber, Hash),
    #[strum(message = "System Number", detailed_message = "value")]
    SystemNumber(BlockNumber),
    #[strum(message = "System ParentHash", detailed_message = "value")]
    SystemParentHash(Hash),
//    #[strum(message = "System RandomSeed", detailed_message = "value")]
//    SystemRandomSeed(Hash),
//    #[strum(message = "System ExtrinsicCount", detailed_message = "value")]
//    SystemExtrinsicCount(u32),
//    #[strum(message = "System ExtrinsicData", detailed_message = "map")]
//    SystemExtrinsicData(u32, Vec<u8>),
//    #[strum(message = "System ExtrinsicsRoot", detailed_message = "value")]
//    SystemExtrinsicsRoot(Hash),
    #[strum(message = "Balances TotalIssuance", detailed_message = "value")]
    BalancesTotalIssuance(Balance),
    #[strum(message = "Balances ExistentialDeposit", detailed_message = "value")]
    BalancesExistentialDeposit(Balance),
    #[strum(message = "Balances ReclaimRebate", detailed_message = "value")]
    BalancesReclaimRebate(Balance),
    #[strum(message = "Balances TransferFee", detailed_message = "value")]
    BalancesTransferFee(Balance),
    #[strum(message = "Balances CreationFee", detailed_message = "value")]
    BalancesCreationFee(Balance),
    #[strum(message = "Balances NextEnumSe", detailed_message = "value")]
    BalancesNextEnumSet(AccountIndex),
    #[strum(message = "Balances EnumSet", detailed_message = "map")]
    BalancesEnumSet(AccountIndex, Vec<AccountId>),
    #[strum(message = "Balances FreeBalance", detailed_message = "map")]
    BalancesFreeBalance(AccountId, Balance),
    #[strum(message = "Balances ReservedBalance", detailed_message = "map")]
    BalancesReservedBalance(AccountId, Balance),
    #[strum(message = "Balances TransactionBaseFee", detailed_message = "value")]
    BalancesTransactionBaseFee(Balance),
    #[strum(message = "Balances TransactionByteFee", detailed_message = "value")]
    BalancesTransactionByteFee(Balance),
    #[strum(message = "Timestamp Now", detailed_message = "value")]
    TimestampNow(Moment),
    #[strum(message = "Timestamp BlockPeriod", detailed_message = "value")]
    TimestampBlockPeriod(Moment),
    #[strum(message = "Session Validators", detailed_message = "value")]
    SessionValidators(Vec<AccountId>),
    #[strum(message = "Session SessionLength", detailed_message = "value")]
    SessionSessionLength(BlockNumber),
    #[strum(message = "Session CurrentIndex", detailed_message = "value")]
    SessionCurrentIndex(BlockNumber),
    #[strum(message = "Session CurrentStart", detailed_message = "value")]
    SessionCurrentStart(Moment),
    #[strum(message = "Session ForcingNewSession", detailed_message = "value")]
    SessionForcingNewSession(bool),
    // chainx
    // XSystem
    #[strum(message = "XSystem BlockProducer", detailed_message = "value")]
    XSystemBlockProducer(AccountId),
    #[strum(message = "XSystem DeathAccount", detailed_message = "value")]
    XSystemDeathAccount(AccountId),
    #[strum(message = "XSystem BannedAccount", detailed_message = "value")]
    XSystemBannedAccount(AccountId),
    #[strum(message = "XSystem BurnAccount", detailed_message = "value")]
    XSystemBurnAccount(AccountId),
    // XAccounts
    #[strum(message = "XAccounts AccountRelationships", detailed_message = "map")]
    XAccountsAccountRelationships(AccountId, AccountId),
    #[strum(message = "XAccounts SharesPerCert", detailed_message = "value")]
    XAccountsSharesPerCert(u32),
    #[strum(message = "XAccounts ActivationPerShare", detailed_message = "value")]
    XAccountsActivationPerShare(u32),
    #[strum(message = "XAccounts MaximumCertCount", detailed_message = "value")]
    XAccountsMaximumCertCount(u32),
    #[strum(message = "XAccounts TotalIssued", detailed_message = "value")]
    XAccountsTotalIssued(u32),
    #[strum(message = "XAccounts CertOwnerOf", detailed_message = "map")]
    XAccountsCertOwnerOf(Vec<u8>, AccountId),
    #[strum(message = "XAccounts CertImmutablePropertiesOf", detailed_message = "map")]
    XAccountsCertImmutablePropertiesOf(Vec<u8>, CertImmutableProps<BlockNumber>),
    #[strum(message = "XAccounts RemainingSharesOf", detailed_message = "map")]
    XAccountsRemainingSharesOf(Vec<u8>, u32),
    #[strum(message = "XAccounts CertNamesOf", detailed_message = "map")]
    XAccountsCertNamesOf(AccountId, Vec<Vec<u8>>),
    #[strum(message = "XAccounts IntentionOf", detailed_message = "map")]
    XAccountsIntentionOf(Vec<u8>, AccountId),
    #[strum(message = "XAccounts IntentionImmutablePropertiesOf", detailed_message = "map")]
    XAccountsIntentionImmutablePropertiesOf(AccountId, IntentionImmutableProps),
    #[strum(message = "XAccounts IntentionPropertiesOf", detailed_message = "map")]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps),
    // fee
    #[strum(message = "XFeeManager Switch", detailed_message = "value")]
    XFeeManagerSwitch(bool),
    // assets
    #[strum(message = "XAssets NativeAssets", detailed_message = "value")]
    XAssetsNativeAssets(Vec<Token>),
    #[strum(message = "XAssets CrossChainAssetsLen", detailed_message = "value")]
    XAssetsCrossChainAssetsLen(u32),
    #[strum(message = "XAssets CrossChainAssets", detailed_message = "map")]
    XAssetsCrossChainAssets(u32, Token),
    #[strum(message = "XAssets AssetInfo", detailed_message = "map")]
    XAssetsAssetInfo(Token, (Asset, bool, BlockNumber)),
    #[strum(message = "XAssets CrossChainAssetsOf", detailed_message = "map")]
    XAssetsCrossChainAssetsOf(AccountId, Vec<Token>),
    #[strum(message = "XAssets TotalXFreeBalance", detailed_message = "map")]
    XAssetsTotalXFreeBalance(Token, Balance),
    #[strum(message = "XAssets XFreeBalance", detailed_message = "map")]
    XAssetsXFreeBalance((AccountId, Token), Balance),
    #[strum(message = "XAssets TotalXReservedBalance", detailed_message = "map")]
    XAssetsTotalXReservedBalance(Token, Balance),
    #[strum(message = "XAssets XReservedBalance", detailed_message = "map")]
    XAssetsXReservedBalance((AccountId, Token, ReservedType), Balance),
    #[strum(message = "XAssets PCXPriceFor", detailed_message = "map")]
    XAssetsPCXPriceFor(Token, Balance),
    #[strum(message = "XAssets RemarkLen", detailed_message = "value")]
    XAssetsRemarkLen(u32),
    #[strum(message = "XAssetsRecords RecordListLenOf", detailed_message = "map")]
    XAssetsRecordsRecordListLenOf(AccountId, u32),
    #[strum(message = "XAssetsRecords RecordListOf", detailed_message = "map")]
    XAssetsRecordsRecordListOf((AccountId, u32), Record<Token, Balance, BlockNumber>),
    #[strum(message = "XAssetsRecords LastDepositIndexOf", detailed_message = "map")]
    XAssetsRecordsLastDepositIndexOf((AccountId, Token), u32),
    #[strum(message = "XAssetsRecords LastWithdrawalIndexOf", detailed_message = "map")]
    XAssetsRecordsLastWithdrawalIndexOf((AccountId, Token), u32),
//    #[strum(message = "XAssetsRecords LogCacheMHeader", detailed_message = "map")]
//    XAssetsRecordsLogCacheMHeader(Token, MultiNodeIndex<Token, WithdrawLog<AccountId>>),
//    #[strum(message = "XAssetsRecords LogCacheMTail", detailed_message = "map")]
//    XAssetsRecordsLogCacheMTail(Token, MultiNodeIndex<Token, WithdrawLog<AccountId>>),
//    #[strum(message = "XAssetsRecords WithdrawLogCache", detailed_message = "map")]
//    XAssetsRecordsWithdrawLogCache((AccountId, u32), Node<WithdrawLog<T::AccountId>>),
    // mining
    // XStaking
    #[strum(message = "XStaking ValidatorCount", detailed_message = "value")]
    XStakingValidatorCount(u32),
    #[strum(message = "XStaking MinimumValidatorCount", detailed_message = "value")]
    XStakingMinimumValidatorCount(u32),
    #[strum(message = "XStaking SessionsPerEra", detailed_message = "value")]
    XStakingSessionsPerEra(BlockNumber),
    #[strum(message = "XStaking OfflineSlash", detailed_message = "value")]
    XStakingOfflineSlash(Amount),
    #[strum(message = "XStaking OfflineSlashGrace", detailed_message = "value")]
    XStakingOfflineSlashGrace(u32),
    #[strum(message = "XStaking BondingDuration", detailed_message = "value")]
    XStakingBondingDuration(BlockNumber),
    #[strum(message = "XStaking CurrentEra", detailed_message = "value")]
    XStakingCurrentEra(BlockNumber),
    #[strum(message = "XStaking Intentions", detailed_message = "value")]
    XStakingIntentions(Vec<AccountId>),
    #[strum(message = "XStaking CurrentSessionReward", detailed_message = "value")]
    XStakingCurrentSessionReward(Balance),
    #[strum(message = "XStaking CurrentOfflineSlash", detailed_message = "value")]
    XStakingCurrentOfflineSlash(Balance),
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
    // XTokens
    #[strum(message = "XTokens PseduIntentions", detailed_message = "value")]
    XTokensPseduIntentions(Vec<Token>),
    #[strum(message = "XTokens PseduIntentionProfiles", detailed_message = "map")]
    XTokensPseduIntentionProfiles(Token, PseduIntentionVoteWeight<Balance, BlockNumber>),
    #[strum(message = "XTokens DepositRecords", detailed_message = "map")]
    XTokensDepositRecords((AccountId, Token), DepositVoteWeight<BlockNumber>),
    // dex
    // XMatchOrder
//    #[strum(message = "XMatchOrder MatchFee", detailed_message = "value")]
//    XMatchOrderMatchFee(Amount),
//    #[strum(message = "XMatchOrder TakerMatchFee", detailed_message = "value")]
//    XMatchOrderTakerMatchFee(Amount),
//    #[strum(message = "XMatchOrder MakerMatchFee", detailed_message = "value")]
//    XMatchOrderMakerMatchFee(Amount),
//    #[strum(message = "XMatchOrder FeePrecision", detailed_message = "value")]
//    XMatchOrderFeePrecision(Amount),
//    #[strum(message = "XMatchOrder BidListHeaderFor", detailed_message = "map")]
//    XMatchOrderBidListHeaderFor((OrderPair,OrderType), MultiNodeIndex<(OrderPair,OrderType), BidT>),
//    #[strum(message = "XMatchOrder BidListTailFor", detailed_message = "map")]
//    XMatchOrderBidListTailFor((OrderPair,OrderType), MultiNodeIndex<(OrderPair,OrderType), BidT>),
//    #[strum(message = "XMatchOrder BidListCache", detailed_message = "map")]
//    XMatchOrderBidListCache(u128, Node<BidT>),
//    #[strum(message = "XMatchOrder NodeId", detailed_message = "value")]
//    XMatchOrderNodeId(u128),
//    #[strum(message = "XMatchOrder BidOf", detailed_message = "map")]
//    XMatchOrderBidOf(BidId, BidDetailT),
//    #[strum(message = "XMatchOrder LastBidIndexOf", detailed_message = "value")]
//    XMatchOrderLastBidIndexOf(BidId),
//    #[strum(message = "XMatchOrder BidOfUserOrder", detailed_message = "map")]
//    XMatchOrderBidOfUserOrder((AccountId, OrderPair, u64), BidId),
//    // XPendingOrders
//    #[strum(message = "XPendingOrders OrderFee", detailed_message = "value")]
//    XPendingOrdersOrderFee(Balance),
//    #[strum(message = "XPendingOrders OrderPairList", detailed_message = "value")]
//    XPendingOrdersOrderPairList(Vec<OrderPair>),
//    #[strum(message = "XPendingOrders OrderPairDetailMap", detailed_message = "map")]
//    XPendingOrdersOrderPairDetailMap(OrderPair, OrderPairDetail),
//    #[strum(message = "XPendingOrders FillIndexOf", detailed_message = "map")]
//    XPendingOrdersFillIndexOf(OrderPair, u128),
//    #[strum(message = "XPendingOrders OrdersOf", detailed_message = "map")]
//    XPendingOrdersOrdersOf((AccountId, OrderPair, u64), OrderT),
//    #[strum(message = "XPendingOrders LastOrderIndexOf", detailed_message = "map")]
//    XPendingOrdersLastOrderIndexOf((AccountId, OrderPair), u64),
//    #[strum(message = "XPendingOrders MaxCommandId", detailed_message = "value")]
//    XPendingOrdersMaxCommandId(u64),
//    #[strum(message = "XPendingOrders CommandOf", detailed_message = "map")]
//    XPendingOrdersCommandOf(u64 , (AccountId, OrderPair, u64, CommandType, u128)),
//    #[strum(message = "XPendingOrders AveragePriceLen", detailed_message = "value")]
//    XPendingOrdersAveragePriceLen(Amount),
//    #[strum(message = "XPendingOrders LastAveragePrice", detailed_message = "map")]
//    XPendingOrdersLastAveragePrice(OrderPair, Price),
//    #[strum(message = "XPendingOrders LastPrice", detailed_message = "map")]
//    XPendingOrdersLastPrice(OrderPair, Price),
//    #[strum(message = "XPendingOrders FeeBuyOrder", detailed_message = "map")]
//    XPendingOrdersFeeBuyOrder(u64, (OrderPair, Amount, Price, AccountId)),
//    #[strum(message = "XPendingOrders FeeBuyOrderMax", detailed_message = "value")]
//    XPendingOrdersFeeBuyOrderMax(u64),
    // bridge
    // BTC
//    #[strum(message = "BridgeOfBTC BestIndex", detailed_message = "value")]
//    BridgeOfBTCBestIndex(BestHeader),
//    #[strum(message = "BridgeOfBTC BlockHeaderFor", detailed_message = "map")]
//    BridgeOfBTCBlockHeaderFor(H256, (BlockHeader, AccountId, BlockNumber)),
//    #[strum(message = "BridgeOfBTC NumberForHash", detailed_message = "map")]
//    BridgeOfBTCNumberForHash(H256, u32),
//    #[strum(message = "BridgeOfBTC HashsForNumber", detailed_message = "map")]
//    BridgeOfBTCHashsForNumber(u32, Vec<H256>),
//    #[strum(message = "BridgeOfBTC GenesisInfo", detailed_message = "value")]
//    BridgeOfBTCGenesisInfo((BlockHeader, u32)),
//    #[strum(message = "BridgeOfBTC ParamsInfo", detailed_message = "value")]
//    BridgeOfBTCParamsInfo(Params),
//    #[strum(message = "BridgeOfBTC NetworkId", detailed_message = "value")]
//    BridgeOfBTCNetworkId(u32),
//    #[strum(message = "BridgeOfBTC TrusteeAddress", detailed_message = "value")]
//    BridgeOfBTCTrusteeAddress(keys::Address),
//    #[strum(message = "BridgeOfBTC TrusteeRedeemScript", detailed_message = "value")]
//    BridgeOfBTCTrusteeRedeemScript(Vec<u8>),
//    #[strum(message = "BridgeOfBTC CertAddress", detailed_message = "value")]
//    BridgeOfBTCCertAddress(keys::Address),
//    #[strum(message = "BridgeOfBTC CertRedeemScript", detailed_message = "value")]
//    BridgeOfBTCCertRedeemScript(Vec<u8>),
//    #[strum(message = "BridgeOfBTC UTXOSet", detailed_message = "value")]
//    BridgeOfBTCUTXOSet(UTXO),
//    #[strum(message = "BridgeOfBTC UTXOSetLen", detailed_message = "value")]
//    BridgeOfBTCUTXOSetLen(u64),
//    #[strum(message = "BridgeOfBTC IrrBlock", detailed_message = "value")]
//    BridgeOfBTCIrrBlock(u32),
//    #[strum(message = "BridgeOfBTC BtcFee", detailed_message = "value")]
//    BridgeOfBTCBtcFee(u64),
//    #[strum(message = "BridgeOfBTC BlockTxsMapKeys", detailed_message = "map")]
//    BridgeOfBTCBlockTxsMapKeys(H256, Vec<H256>),
//    #[strum(message = "BridgeOfBTC AddressMap", detailed_message = "map")]
//    BridgeOfBTCAddressMap(keys::Address, BindInfo<AccountId>),
//    #[strum(message = "BridgeOfBTC TxProposalLen", detailed_message = "value")]
//    BridgeOfBTCTxProposalLen(u32),
//    #[strum(message = "BridgeOfBTC TxProposal", detailed_message = "map")]
//    BridgeOfBTCTxProposal(u32, Vec<DepositInfo<AccountId>>),
//    #[strum(message = "BridgeOfBTC DepositCache", detailed_message = "value")]
//    BridgeOfBTCDepositCache(Vec<DepositInfo<AccountId>>),
//    #[strum(message = "BridgeOfBTC DepositRecordsMap", detailed_message = "map")]
//    BridgeOfBTCDepositRecordsMap(Address, Vec<DepositHistInfo>),
//    #[strum(message = "BridgeOfBTC CertCache", detailed_message = "value")]
//    BridgeOfBTCCertCache(Vec<(Vec<u8>, u32, AccountId)>),
//    #[strum(message = "BridgeOfBTC Fee", detailed_message = "value")]
//    BridgeOfBTCFee(Balance),
    #[strum(message = "Invalid", detailed_message = "value")]
    Invalid,
}

macro_rules! to_value_json {
    ($prefix:ident, $value:ident => $v:ident) => {
        {
            *$v = Decode::decode(&mut $value.as_slice())
                    .ok_or(format!("Decode failed, prefix: {:?}, value: {:?}", $prefix, $v))?;
            Ok(json!({
                "type":"value",
                "prefix":$prefix,
                "key":null,
                "value":$v,
            }))
        }
    };
}

macro_rules! to_map_json {
    ($prefix:ident, $key:ident => $k:ident, $value:ident => $v:ident) => {
        {
            *$k = Decode::decode(&mut $key.as_bytes())
                    .ok_or(format!("Decode failed, prefix: {:?}, key: {:?}", $prefix, $k))?;
            *$v = Decode::decode(&mut $value.as_slice())
                    .ok_or(format!("Decode failed, prefix: {:?}, key: {:?}, value: {:?}", $prefix, $k, $v))?;
            Ok(json!({
                "type":"map",
                "prefix":$prefix,
                "key":$k,
                "value":$v,
            }))
        }
    };
}

impl RuntimeStorage {
    pub fn parse(key: &str, value: Vec<u8>) -> Result<(String, serde_json::Value)> {
        let (mut storage, prefix) = Self::match_key(key)?;
        Ok((prefix.clone(), storage.decode_by_type(prefix, key, value)?))
    }

    pub fn match_key(key: &str) -> Result<(Self, String)> {
        for storage in Self::iter() {
            let prefix: String = storage
                .get_message()
                .ok_or("Get storage prefix failed".to_string())?
                .into();
            if key.starts_with(&prefix) {
                return Ok((storage, prefix));
            }
        }
        Err("No matching key found".into())
    }

    #[rustfmt::skip]
    pub fn decode_by_type(&mut self, prefix: String, key: &str, value: Vec<u8>) -> Result<serde_json::Value> {
        let key = match self.get_detailed_message() {
            Some("map") => &key[prefix.len()..],
            Some("value") => key,
            _ => return Err("Invalid storage type".into()),
        };

        match self {
            // substrate
            RuntimeStorage::SystemAccountNonce(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::SystemBlockHash(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::SystemNumber(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SystemParentHash(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::SystemRandomSeed(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::SystemExtrinsicCount(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::SystemExtrinsicData(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::SystemExtrinsicsRoot(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTotalIssuance(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesExistentialDeposit(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesReclaimRebate(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTransferFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesCreationFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesNextEnumSet(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesEnumSet(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesFreeBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesReservedBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::BalancesTransactionBaseFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::BalancesTransactionByteFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::TimestampNow(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::TimestampBlockPeriod(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionValidators(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionSessionLength(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentStart(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionForcingNewSession(ref mut v) => to_value_json!(prefix, value => v),
            // chainx
            RuntimeStorage::XSystemBlockProducer(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemDeathAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemBannedAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemBurnAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsAccountRelationships(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
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
            RuntimeStorage::XAssetsNativeAssets(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsCrossChainAssetsLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsCrossChainAssets(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsAssetInfo(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsCrossChainAssetsOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsTotalXFreeBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsXFreeBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsTotalXReservedBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsXReservedBalance(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsPCXPriceFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRemarkLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsRecordsRecordListLenOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsRecordListOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsLastDepositIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsLastWithdrawalIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsLogCacheMHeader(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsLogCacheMTail(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsWithdrawLogCache(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingMinimumValidatorCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingOfflineSlash(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingOfflineSlashGrace(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingBondingDuration(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCurrentEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCurrentSessionReward(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingCurrentOfflineSlash(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingNextSessionsPerEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingLastEraLengthChange(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingForcingNewEra(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XStakingStakeWeight(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XStakingNominationRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensPseduIntentions(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XTokensPseduIntentionProfiles(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XTokensDepositRecords(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderMatchFee(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderTakerMatchFee(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderMakerMatchFee(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderFeePrecision(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderBidListHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderBidListTailFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderBidListCache(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderNodeId(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderBidOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderLastBidIndexOf(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderBidOfUserOrder(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersOrderFee(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XPendingOrdersOrderPairList(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XPendingOrdersOrderPairDetailMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersFillIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersOrdersOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersLastOrderIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersMaxCommandId(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XPendingOrdersCommandOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersAveragePriceLen(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XPendingOrdersLastAveragePrice(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersLastPrice(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersFeeBuyOrder(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XPendingOrdersFeeBuyOrderMax(ref mut v) => to_value_json!(prefix, value => v),
            // bridge
            // BTC
//            RuntimeStorage::BridgeOfBTCBestIndex(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCBlockHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCNumberForHash(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCHashsForNumber(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCGenesisInfo(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCParamsInfo(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCNetworkId(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCTrusteeAddress(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCTrusteeRedeemScript(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCCertAddress(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCCertRedeemScript(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCUTXOSet(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCUTXOSetLen(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCIrrBlock(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCBtcFee(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCBlockTxsMapKeys(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCAddressMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCTxProposalLen(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCTxProposal(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCDepositCache(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCDepositRecordsMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::BridgeOfBTCCertCache(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::BridgeOfBTCFee(ref mut v) => to_value_json!(prefix, value => v),
            invalid @ _ => Err(format!("Invalid Runtime Storage: {:?}", invalid).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    pub fn test_parse_match_value() {
        let key = "Balances TotalIssuance";
        let value = vec![123u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let got = RuntimeStorage::parse(key, value).unwrap();
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
        let key = "Balances FreeBalance12345678901234567890123456789012";
        let value = vec![123u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let got = RuntimeStorage::parse(key, value).unwrap();
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
        let key = "XAssets AssetInfo\u{c}PCX";
        let value = vec![
            12, 80, 67, 88, 0, 3, 0, 68, 80, 67, 88, 32, 111, 110, 99, 104, 97, 105, 110, 32, 116,
            111, 107, 101, 110, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let got = RuntimeStorage::parse(key, value).unwrap();
        let exp = serde_json::Value::from_str(
            r#"{
                "type":"map",
                "prefix":"XAssets AssetInfo",
                "key":[80, 67, 88],
                "value":[
                    {
                        "token":[80, 67, 88],
                        "chain":"PCX",
                        "precision":3,
                        "desc":[80, 67, 88, 32, 111, 110, 99, 104, 97, 105, 110, 32, 116, 111, 107, 101, 110]
                    },
                    true,
                    0
                ]
            }"#,
        ).unwrap();
        assert_eq!(got, exp);
    }
}
