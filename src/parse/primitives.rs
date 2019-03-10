use parity_codec::Codec;
use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

use crate::types::{btc, Bytes, NodeT};

// ================================================================================================
// Substrate primitives.
// ================================================================================================

pub use substrate_primitives::H256;

//pub use srml_balances::VestingSchedule;
/// Struct to encode the vesting schedule of an individual account.
#[derive(Copy, Clone, PartialEq, Eq, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct VestingSchedule<Balance>
where
    Balance: Copy + Default + Codec,
{
    /// Locked amount at genesis.
    pub offset: Balance,
    /// Amount that gets unlocked every block from genesis.
    pub per_block: Balance,
}

/// A hash of some data used by the relay chain.
pub type Hash = substrate_primitives::H256;

/// The Ed25519 pub key of an session that belongs to an authority of the relay chain. This is
/// exactly equivalent to what the substrate calls an "authority".
pub type SessionKey = substrate_primitives::Ed25519AuthorityId;

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
/// TODO: switch to u32
pub type BlockNumber = u64;

/// Alias to Ed25519 pubkey that identifies an account on the relay chain.
pub type AccountId = substrate_primitives::H256;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Index = u64;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// The balance of an account.
/// u64 for chainx token and all assets type, if the asset is not suit for u64, choose a suitable precision
pub type Balance = u64;

// ================================================================================================
// ChainX primitives.
// ================================================================================================

pub type XString = String;

// ============================================================================
// xaccounts runtime module definitions.
// ============================================================================

pub type Name = XString;
pub type URL = XString;

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IntentionProps {
    pub url: URL,
    pub is_active: bool,
    pub about: XString,
    //    pub session_key: Option<SessionKey>, # TODO: add the field in next testnet version.
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum TrusteeEntity {
    Bitcoin(Bytes),
}

impl Default for TrusteeEntity {
    fn default() -> Self {
        TrusteeEntity::Bitcoin(Bytes::default())
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeIntentionProps {
    pub about: XString,
    pub hot_entity: TrusteeEntity,
    pub cold_entity: TrusteeEntity,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeAddressPair {
    pub hot_address: Bytes,
    pub cold_address: Bytes,
}

// ============================================================================
// xfee - manager runtime module definitions.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SwitchStore {
    pub global: bool,
    pub spot: bool,
    pub xbtc: bool,
    pub sdot: bool,
}

// ============================================================================
// xassets - assets runtime module definitions.
// ============================================================================

pub type Token = XString;
pub type Desc = XString;
pub type Precision = u16;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum Chain {
    ChainX,
    Bitcoin,
    Ethereum,
    Polkadot,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::ChainX
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Asset {
    token: Token,
    token_name: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum AssetType {
    Free,
    ReservedStaking,
    ReservedStakingRevocation,
    ReservedWithdrawal,
    ReservedDexSpot,
    ReservedDexFuture,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Free
    }
}

// ============================================================================
// xassets - records runtime module definitions.
// ============================================================================

pub type AddrStr = XString;
pub type Memo = XString;

/// application for withdrawal
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Application<AccountId, Balance, Moment>
where
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    Moment: Copy + Default + Codec,
{
    id: u32,
    applicant: AccountId,
    token: Token,
    balance: Balance,
    addr: AddrStr,
    ext: Memo,
    time: Moment,
}

impl<AccountId, Balance> NodeT for Application<AccountId, Balance, Moment>
where
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    Moment: Copy + Default + Codec,
{
    type Index = u32;
    fn index(&self) -> Self::Index {
        self.id
    }
}

// ============================================================================
// xmining - staking runtime module definitions.
// ============================================================================

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IntentionProfs<Balance, BlockNumber>
where
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub total_nomination: Balance,
    pub last_total_vote_weight: u64,
    pub last_total_vote_weight_update: BlockNumber,
}

/// Nomination record of one of the nominator's nominations.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct NominationRecord<Balance, BlockNumber>
where
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub nomination: Balance,
    pub last_vote_weight: u64,
    pub last_vote_weight_update: BlockNumber,
    pub revocations: Vec<(BlockNumber, Balance)>,
}

// ============================================================================
// xmining - tokens runtime module definitions.
// ============================================================================

/// This module only tracks the vote weight related changes.
/// All the amount related has been taken care by assets module.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct PseduIntentionVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct DepositVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}

// ============================================================================
// xdex - spot runtime module definitions.
// ============================================================================

pub type Price = Balance;
pub type Amount = Balance;

pub type ID = u64;
pub type TradingPairIndex = u32;

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CurrencyPair(pub Token, pub Token); // base currency / counter currency

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TradingPair {
    pub id: TradingPairIndex,
    pub currency_pair: CurrencyPair,
    pub precision: u32,      // price precision
    pub unit_precision: u32, // minimum unit precision
    pub online: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum OrderType {
    Limit,
    Market,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum OrderDirection {
    Buy,
    Sell,
}

impl Default for OrderDirection {
    fn default() -> Self {
        OrderDirection::Buy
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum OrderStatus {
    ZeroExecuted,
    ParitialExecuted,
    AllExecuted,
    ParitialExecutedAndCanceled,
    Canceled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::ZeroExecuted
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct OrderProperty<
    Pair: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Amount: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
>(
    AccountId,
    Pair,
    OrderDirection,
    Amount,
    Price,
    ID,
    OrderType,
    BlockNumber,
);

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Order<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Amount: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    props: OrderProperty<Pair, AccountId, Amount, Price, BlockNumber>,

    pub status: OrderStatus,
    pub remaining: Amount,   // remaining amount, measured by counter currency
    pub fill_index: Vec<ID>, // index of transaction record
    pub already_filled: Amount,
    pub last_update_at: BlockNumber, // FIXME BlockNumber or Timestamp?
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Handicap<Price>
where
    Price: Copy + Default + Codec,
{
    pub buy: Price,
    pub sell: Price,
}

// ============================================================================
// xbridge - bitcoin runtime module definitions.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BlockHeaderInfo {
    pub header: btc::BlockHeader,
    pub height: u32,
    pub confirmed: bool,
    pub txid_list: Vec<H256>,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum TxType {
    Withdraw,
    Deposit,
}

impl Default for TxType {
    fn default() -> Self {
        TxType::Deposit
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TxInfo {
    pub raw_tx: btc::Transaction,
    pub tx_type: TxType,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct DepositCache {
    pub txid: H256,
    pub balance: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum VoteResult {
    Unfinish,
    Finish,
}

impl Default for VoteResult {
    fn default() -> Self {
        VoteResult::Unfinish
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CandidateTx<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub sig_state: VoteResult,
    pub withdrawal_id_list: Vec<u32>,
    pub tx: btc::Transaction,
    pub trustee_list: Vec<(AccountId, bool)>,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Params {
    max_bits: u32,
    //Compact
    block_max_future: u32,

    target_timespan_seconds: u32,
    target_spacing_seconds: u32,
    retargeting_factor: u32,

    retargeting_interval: u32,
    min_timespan: u32,
    max_timespan: u32,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeScriptInfo {
    pub hot_redeem_script: Bytes,
    pub cold_redeem_script: Bytes,
}

// ============================================================================
// xbridge - sdot runtime module definitions.
// ============================================================================

pub type EthereumAddress = substrate_primitives::H160;
