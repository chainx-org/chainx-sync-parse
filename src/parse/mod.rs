//mod metadata;
mod primitives;

use parity_codec::Decode;
use strum::{EnumMessage, IntoEnumIterator};

//pub use self::metadata::{get_runtime_metadata, parse_metadata};
pub use self::primitives::*;

#[derive(EnumIter, EnumMessage, Debug)]
pub enum RuntimeStorage {
    #[strum(message = "System AccountNonce", detailed_message = "Map")]
    SystemAccountNonce { k: AccountId, v: Index },
    #[strum(message = "System BlockHash", detailed_message = "Map")]
    SystemBlockHash { k: BlockNumber, v: Hash },
    #[strum(message = "Timestamp Now", detailed_message = "Value")]
    TimestampNow { v: Timestamp },
    #[strum(message = "Timestamp BlockPeriod", detailed_message = "Value")]
    TimestampBlockPeriod { v: Timestamp },
    #[strum(message = "Balances TotalIssuance", detailed_message = "Value")]
    BalancesTotalIssuance { v: Balance },
    #[strum(message = "Balances ExistentialDeposit", detailed_message = "Value")]
    BalancesExistentialDeposit { v: Balance },
    #[strum(message = "Balances ReclaimRebate", detailed_message = "Value")]
    BalancesReclaimRebate { v: Balance },
    #[strum(message = "Balances TransferFee", detailed_message = "Value")]
    BalancesTransferFee { v: Balance },
    #[strum(message = "Balances CreationFee", detailed_message = "Value")]
    BalancesCreationFee { v: Balance },
    #[strum(message = "Balances NextEnumSe", detailed_message = "Value")]
    BalancesNextEnumSet { v: AccountIndex },
    #[strum(message = "Balances EnumSet", detailed_message = "Map")]
    BalancesEnumSet { k: AccountIndex, v: Vec<AccountId> },
    #[strum(message = "Balances FreeBalance", detailed_message = "Map")]
    BalancesFreeBalance { k: AccountId, v: Balance },
    #[strum(message = "Balances ReservedBalance", detailed_message = "Map")]
    BalancesReservedBalance { k: AccountId, v: Balance },
    #[strum(message = "Balances TransactionBaseFee", detailed_message = "Value")]
    BalancesTransactionBaseFee { v: Balance },
    #[strum(message = "Balances TransactionByteFee", detailed_message = "Value")]
    BalancesTransactionByteFee { v: Balance },
    #[strum(message = "Session Validators", detailed_message = "Value")]
    SessionValidators { v: Vec<AccountId> },
    #[strum(message = "Session SessionLength", detailed_message = "Value")]
    SessionSessionLength { v: BlockNumber },
    #[strum(message = "Session CurrentIndex", detailed_message = "Value")]
    SessionCurrentIndex { v: BlockNumber },
    #[strum(message = "Session CurrentStart", detailed_message = "Value")]
    SessionCurrentStart { v: Timestamp },
    #[strum(message = "Session ForcingNewSession", detailed_message = "Value")]
    SessionForcingNewSession { v: Option<bool> },
}

macro_rules! decode_key {
    ($key:ident => $k:ident) => {
        *$k = Decode::decode(&mut $key.as_bytes()).unwrap();
    };
}

macro_rules! decode_value {
    ($value:ident => $v:ident) => {
        *$v = Decode::decode(&mut $value.as_slice()).unwrap();
    };
}

macro_rules! decode_map {
    ($key:ident => $k:ident, $value:ident => $v:ident) => {
        decode_key!($key => $k);
        decode_value!($value => $v);
    };
}

impl RuntimeStorage {
    pub fn new(key: &str) -> Option<Self> {
        for storage in RuntimeStorage::iter() {
            if key.starts_with(storage.get_message().unwrap()) {
                return Some(storage);
            }
        }
        None
    }

    #[rustfmt::skip]
    pub fn parse_match(&mut self, key: &str, value: Vec<u8>) {
        let prefix_len: usize = self.get_message().unwrap().len();

        let key = match self.get_detailed_message().unwrap() {
            "Map" => &key[prefix_len..],
            _ => key,
        };

        match self {
            RuntimeStorage::SystemAccountNonce { ref mut k, ref mut v } => { decode_map!(key => k, value => v); }
            RuntimeStorage::SystemBlockHash { ref mut k, ref mut v } => { decode_map!(key => k, value => v); }
            RuntimeStorage::TimestampNow { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::TimestampBlockPeriod { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesTotalIssuance { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesExistentialDeposit { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesReclaimRebate { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesTransferFee { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesCreationFee { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesNextEnumSet { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesEnumSet { ref mut k, ref mut v } => { decode_map!(key => k, value => v); }
            RuntimeStorage::BalancesFreeBalance { ref mut k, ref mut v } => { decode_map!(key => k, value => v); }
            RuntimeStorage::BalancesReservedBalance { ref mut k, ref mut v } => { decode_map!(key => k, value => v); }
            RuntimeStorage::BalancesTransactionBaseFee { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::BalancesTransactionByteFee { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::SessionValidators { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::SessionSessionLength { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::SessionCurrentIndex { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::SessionCurrentStart { ref mut v } => { decode_value!(value => v); }
            RuntimeStorage::SessionForcingNewSession { ref mut v } => { decode_value!(value => v); }
        }

        println!("self: {:?}", self);
    }
}

pub fn test_parse_match() {
    let key = "Balances FreeBalance((((((((((((((((((((((((((((((((";
    let value = vec![40u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut free_balance = RuntimeStorage::new(key);
    free_balance.unwrap().parse_match(key, value);
}
