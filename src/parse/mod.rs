//mod metadata;
mod primitives;

use parity_codec::Decode;
use strum::{EnumMessage, IntoEnumIterator};

use std::str::FromStr;

use error::Result;
use serde_ext::Bytes;
//pub use self::metadata::{get_runtime_metadata, parse_metadata};
pub use self::primitives::*;

#[rustfmt::skip]
#[derive(EnumIter, EnumMessage, Debug, Eq, PartialEq)]
pub enum RuntimeStorage {
    // substrate
    #[strum(message = "System AccountNonce", detailed_message = "map")]
    SystemAccountNonce(AccountId, Index),
    #[strum(message = "System BlockHash", detailed_message = "map")]
    SystemBlockHash(BlockNumber, Hash),
    #[strum(message = "Timestamp Now", detailed_message = "value")]
    TimestampNow(Timestamp),
    #[strum(message = "Timestamp BlockPeriod", detailed_message = "value")]
    TimestampBlockPeriod(Timestamp),
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
    #[strum(message = "Session Validators", detailed_message = "value")]
    SessionValidators(Vec<AccountId>),
    #[strum(message = "Session SessionLength", detailed_message = "value")]
    SessionSessionLength(BlockNumber),
    #[strum(message = "Session CurrentIndex", detailed_message = "value")]
    SessionCurrentIndex(BlockNumber),
    #[strum(message = "Session CurrentStart", detailed_message = "value")]
    SessionCurrentStart(Timestamp),
    #[strum(message = "Session ForcingNewSession", detailed_message = "value")]
    SessionForcingNewSession(Option<bool>),
    // chainx
    #[strum(message = "XSystem BlockProducer", detailed_message = "value")]
    XSystemBlockProducer(Option<AccountId>),
    #[strum(message = "XSystem DeathAccount", detailed_message = "value")]
    XSystemDeathAccount(AccountId),
    #[strum(message = "XSystem BurnAccount", detailed_message = "value")]
    XSystemBurnAccount(AccountId),
    #[strum(message = "XAccounts AccountRelationships", detailed_message = "value")]
    XAccountsAccountRelationships(AccountId, Option<AccountId>),
    #[strum(message = "XAccounts SharesPerCert", detailed_message = "value")]
    XAccountsSharesPerCert(u32),
    #[strum(message = "XAccounts ActivationPerShare", detailed_message = "value")]
    XAccountsActivationPerShare(u32),
    #[strum(message = "XAccounts MaximumCertCount", detailed_message = "value")]
    XAccountsMaximumCertCount(u32),
    #[strum(message = "XAccounts TotalIssued", detailed_message = "value")]
    XAccountsTotalIssued(u32),
    #[strum(message = "XAccounts CertOwnerOf", detailed_message = "map")]
    XAccountsCertOwnerOf(Vec<u8>, Option<AccountId>),
    #[strum(message = "XAccounts Certs", detailed_message = "value")]
    XAccountsCerts(Vec<Vec<u8>>),
    #[strum(message = "XAccounts CertImmutablePropertiesOf", detailed_message = "map")]
    XAccountsCertImmutablePropertiesOf(Vec<u8>, CertImmutableProps<BlockNumber>),
    #[strum(message = "XAccounts RemainingSharesOf", detailed_message = "map")]
    XAccountsRemainingSharesOf(Vec<u8>, u32),
    #[strum(message = "XAccounts CertNamesOf", detailed_message = "map")]
    XAccountsCertNamesOf(AccountId, Vec<Vec<u8>>),
    #[strum(message = "XAccounts IntentionOf", detailed_message = "map")]
    XAccountsIntentionOf(Vec<u8>, Option<AccountId>),
    #[strum(message = "XAccounts IntentionImmutablePropertiesOf", detailed_message = "map")]
    XAccountsIntentionImmutablePropertiesOf(AccountId, Option<IntentionImmutableProps>),
    #[strum(message = "XAccounts IntentionPropertiesOf", detailed_message = "map")]
    XAccountsIntentionPropertiesOf(AccountId, IntentionProps),
    #[strum(message = "XFeeManager Switch", detailed_message = "value")]
    XFeeManagerSwitch(bool),
    #[strum(message = "XAssets NativeAssets", detailed_message = "value")]
    XAssetsNativeAssets(Vec<Token>),
    #[strum(message = "XAssets CrossChainAssetsLen", detailed_message = "value")]
    XAssetsCrossChainAssetsLen(u32),
    #[strum(message = "XAssets CrossChainAssets", detailed_message = "value")]
    XAssetsCrossChainAssets(u32, Token),
    #[strum(message = "XAssets AssetInfo", detailed_message = "map")]
    XAssetsAssetInfo(Token, Option<(Asset, bool, BlockNumber)>),
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
    XAssetsPCXPriceFor(Token, Option<Balance>),
    #[strum(message = "XAssets RemarkLen", detailed_message = "value")]
    XAssetsRemarkLen(u32),
    #[strum(message = "XAssets RecordsRecordListLenOf", detailed_message = "map")]
    XAssetsRecordsRecordListLenOf(AccountId, u32),
    #[strum(message = "XAssets RecordsRecordListOf", detailed_message = "value")]
    XAssetsRecordsRecordListOf(Option<Record<Token, Balance, BlockNumber>>),
    #[strum(message = "XAssets RecordsLastDepositIndexOf", detailed_message = "map")]
    XAssetsRecordsLastDepositIndexOf((AccountId, Token), Option<u32>),
    #[strum(message = "XAssets RecordsLastWithdrawalIndexOf", detailed_message = "map")]
    XAssetsRecordsLastWithdrawalIndexOf((AccountId, Token), Option<u32>),
//    #[strum(message = "XAssets RecordsLogCacheMHeader", detailed_message = "map")]
//    XAssetsRecordsLogCacheMHeader(Token, Option<MultiNodeIndex<Token, WithdrawLog<AccountId>>>),
//    #[strum(message = "XAssets RecordsLogCacheMTail", detailed_message = "map")]
//    XAssetsRecordsLogCacheMTail(Token, Option<MultiNodeIndex<Token, WithdrawLog<AccountId>>>),
//    #[strum(message = "XAssets RecordsWithdrawLogCache", detailed_message = "map")]
//    XAssetsRecordsWithdrawLogCache((AccountId, u32), Option<MultiNodeIndex<Token, WithdrawLog<AccountId>>>),
    #[strum(message = "XMatchOrder MatchFee", detailed_message = "value")]
    XMatchOrderMatchFee(Amount),
    #[strum(message = "XMatchOrder TakerMatchFee", detailed_message = "value")]
    XMatchOrderTakerMatchFee(Amount),
    #[strum(message = "XMatchOrder MakerMatchFee", detailed_message = "value")]
    XMatchOrderMakerMatchFee(Amount),
    #[strum(message = "XMatchOrder FeePrecision", detailed_message = "value")]
    XMatchOrderFeePrecision(Amount),
//    #[strum(message = "XMatchOrder BidListHeaderFor", detailed_message = "map")]
//    XMatchOrderBidListHeaderFor((OrderPair,OrderType), Option<MultiNodeIndex<(OrderPair,OrderType), BidT>>),
//    #[strum(message = "XMatchOrder BidListTailFor", detailed_message = "map")]
//    XMatchOrderBidListTailFor((OrderPair,OrderType), Option<MultiNodeIndex<(OrderPair,OrderType), BidT>>),
//    #[strum(message = "XMatchOrder BidListCache", detailed_message = "map")]
//    XMatchOrderBidListCache(u128, Option<Node<BidT>>),
    #[strum(message = "XMatchOrder NodeId", detailed_message = "value")]
    XMatchOrderNodeId(u128),
    #[strum(message = "XMatchOrder BidOf", detailed_message = "map")]
    XMatchOrderBidOf(BidId, Option<BidDetailT>),
    #[strum(message = "XMatchOrder LastBidIndexOf", detailed_message = "value")]
    XMatchOrderLastBidIndexOf(BidId),
    #[strum(message = "XMatchOrder BidOfUserOrder", detailed_message = "map")]
    XMatchOrderBidOfUserOrder((AccountId, OrderPair, u64), BidId),
    #[strum(message = "XPendingOrders OrderFee", detailed_message = "value")]
    XPendingOrdersOrderFee(Balance),
    #[strum(message = "XPendingOrders OrderPairList", detailed_message = "value")]
    XPendingOrdersOrderPairList(Vec<OrderPair>),
    #[strum(message = "XPendingOrders OrderPairDetailMap", detailed_message = "map")]
    XPendingOrdersOrderPairDetailMap(OrderPair, Option<OrderPairDetail>),
    #[strum(message = "XPendingOrders FillIndexOf", detailed_message = "map")]
    XPendingOrdersFillIndexOf(OrderPair, u128),
    #[strum(message = "XPendingOrders OrdersOf", detailed_message = "map")]
    XPendingOrdersOrdersOf((AccountId, OrderPair, u64), Option<OrderT>),
    #[strum(message = "XPendingOrders LastOrderIndexOf", detailed_message = "map")]
    XPendingOrdersLastOrderIndexOf((AccountId, OrderPair), Option<u64>),
    #[strum(message = "XPendingOrders MaxCommandId", detailed_message = "value")]
    XPendingOrdersMaxCommandId(u64),
    #[strum(message = "XPendingOrders CommandOf", detailed_message = "map")]
    XPendingOrdersCommandOf(u64 , Option<(AccountId, OrderPair, u64, CommandType, u128)>),
    #[strum(message = "XPendingOrders AveragePriceLen", detailed_message = "value")]
    XPendingOrdersAveragePriceLen(Amount),
    #[strum(message = "XPendingOrders LastAveragePrice", detailed_message = "map")]
    XPendingOrdersLastAveragePrice(OrderPair, Option<Price>),
    #[strum(message = "XPendingOrders LastPrice", detailed_message = "map")]
    XPendingOrdersLastPrice(OrderPair, Option<Price>),
    #[strum(message = "XPendingOrders FeeBuyOrder", detailed_message = "map")]
    XPendingOrdersFeeBuyOrder(u64, Option<(OrderPair, Amount, Price, AccountId)>),
    #[strum(message = "XPendingOrders FeeBuyOrderMax", detailed_message = "value")]
    XPendingOrdersFeeBuyOrderMax(u64),
}

macro_rules! to_value_json {
    ($prefix:ident, $value:ident => $v:ident) => {
        {
            *$v = Decode::decode(&mut $value.as_slice()).unwrap();
            json!({
                "type":"value",
                "prefix":$prefix,
                "key":null,
                "value":$v,
            })
        }
    };
}

macro_rules! to_map_json {
    ($prefix:ident, $key:ident => $k:ident, $value:ident => $v:ident) => {
        {
            *$k = Decode::decode(&mut $key.as_bytes()).unwrap();
            *$v = Decode::decode(&mut $value.as_slice()).unwrap();
            json!({
                "type":"map",
                "prefix":$prefix,
                "key":$k,
                "value":$v,
            })
        }
    };
}

impl RuntimeStorage {
    pub fn parse(key: &str, value: Vec<u8>) -> Result<serde_json::Value> {
        let (mut storage, prefix) = Self::match_key(key)?;
        Ok(storage.decode_by_type(prefix, key, value))
    }

    fn match_key(key: &str) -> Result<(Self, String)> {
        for storage in Self::iter() {
            let prefix: String = storage.get_message().unwrap().into();
            if key.starts_with(&prefix) {
                return Ok((storage, prefix));
            }
        }
        Err("No matching key found".into())
    }

    #[rustfmt::skip]
    fn decode_by_type(&mut self, prefix: String, key: &str, value: Vec<u8>) -> serde_json::Value {
        let key = match self.get_detailed_message().unwrap() {
            "map" => &key[prefix.len()..],
            _ => key,
        };

        match self {
            // substrate
            RuntimeStorage::SystemAccountNonce(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::SystemBlockHash(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::TimestampNow(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::TimestampBlockPeriod(ref mut v) => to_value_json!(prefix, value => v),
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
            RuntimeStorage::SessionValidators(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionSessionLength(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentIndex(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionCurrentStart(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::SessionForcingNewSession(ref mut v) => to_value_json!(prefix, value => v),
            // chainx
            RuntimeStorage::XSystemBlockProducer(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemDeathAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XSystemBurnAccount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsAccountRelationships(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsSharesPerCert(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsActivationPerShare(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsMaximumCertCount(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsTotalIssued(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAccountsCertOwnerOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAccountsCerts(ref mut v) => to_value_json!(prefix, value => v),
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
            RuntimeStorage::XAssetsRecordsRecordListOf(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XAssetsRecordsLastDepositIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XAssetsRecordsLastWithdrawalIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsLogCacheMHeader(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsLogCacheMTail(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XAssetsRecordsWithdrawLogCache(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMatchOrderMatchFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMatchOrderTakerMatchFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMatchOrderMakerMatchFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMatchOrderFeePrecision(ref mut v) => to_value_json!(prefix, value => v),
//            RuntimeStorage::XMatchOrderBidListHeaderFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderBidListTailFor(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
//            RuntimeStorage::XMatchOrderBidListCache(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMatchOrderNodeId(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMatchOrderBidOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XMatchOrderLastBidIndexOf(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XMatchOrderBidOfUserOrder(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersOrderFee(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XPendingOrdersOrderPairList(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XPendingOrdersOrderPairDetailMap(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersFillIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersOrdersOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersLastOrderIndexOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersMaxCommandId(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XPendingOrdersCommandOf(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersAveragePriceLen(ref mut v) => to_value_json!(prefix, value => v),
            RuntimeStorage::XPendingOrdersLastAveragePrice(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersLastPrice(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersFeeBuyOrder(ref mut k, ref mut v) => to_map_json!(prefix, key => k, value => v),
            RuntimeStorage::XPendingOrdersFeeBuyOrderMax(ref mut v) => to_value_json!(prefix, value => v),
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

    //    #[test]
    //    fn test_serde_json_128() {
    //        use std::str::FromStr;
    //        assert_eq!(
    //            serde_json::Value::from_str(&format!("{}", u128::max_value())).unwrap(),
    //            serde_json::to_value(u128::max_value()).unwrap(),
    //        );
    //        assert_eq!(
    //            serde_json::Value::from_str(&format!("{}", i128::max_value())).unwrap(),
    //            serde_json::to_value(i128::max_value()).unwrap(),
    //        );
    //    }
}
