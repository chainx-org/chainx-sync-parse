use parity_codec::Codec;

use super::btc;
use super::linked_node::NodeT;

// ================================================================================================
// Substrate primitives.
// ================================================================================================

pub use sr_primitives::Perbill;
pub use substrate_primitives::H256;

/// A hash of some data used by the relay chain.
pub type Hash = substrate_primitives::H256;

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
/// TODO: switch to u32
pub type BlockNumber = u64;

/// Alias to Ed25519 pubkey that identifies an account on the relay chain.
pub type AccountId = substrate_primitives::H256;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u64;

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

/// Cert immutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CertImmutableProps<BlockNumber: Default, Moment: Default> {
    pub issued_at: (BlockNumber, Moment),
    pub frozen_duration: u32,
}

/// Intention Immutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IntentionImmutableProps<Moment> {
    pub name: Name,
    pub activator: Name,
    pub initial_shares: u32,
    pub registered_at: Moment,
}

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IntentionProps {
    pub url: URL,
    pub is_active: bool,
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
pub struct Application<AccountId, Balance>
where
    AccountId: Codec + Clone,
    Balance: Codec + Copy,
{
    id: u32,
    applicant: AccountId,
    token: Token,
    balance: Balance,
    addr: AddrStr,
    ext: Memo,
}

impl<AccountId, Balance> NodeT for Application<AccountId, Balance>
where
    AccountId: Codec + Clone,
    Balance: Codec + Copy,
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
pub struct IntentionProfs<Balance: Default, BlockNumber: Default> {
    pub jackpot: Balance,
    pub total_nomination: Balance,
    pub last_total_vote_weight: u64,
    pub last_total_vote_weight_update: BlockNumber,
}

/// Nomination record of one of the nominator's nominations.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct NominationRecord<Balance: Default, BlockNumber: Default> {
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
pub struct PseduIntentionVoteWeight<Balance: Default, BlockNumber: Default> {
    pub jackpot: Balance,
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct DepositVoteWeight<BlockNumber: Default> {
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}

// ============================================================================
// xdex - spot runtime module definitions.
// ============================================================================

pub type Price = u128;

pub type ID = u128;
pub type OrderPairID = u32;

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct OrderPair {
    pub id: OrderPairID,
    pub first: Token,
    pub second: Token,
    pub precision: u32, //价格精度
    pub used: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum OrderType {
    Limit,  //限价单
    Market, //市价单
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
    FillNo,
    FillPart,
    FillAll,
    FillPartAndCancel,
    Cancel,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::FillNo
    }
}

/// 用户的委托记录 包含了成交历史的index
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Order<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone,
    AccountId: Clone,
    Amount: Copy,
    Price: Copy,
    BlockNumber: Copy,
{
    pub pair: Pair,
    pub price: Price,
    pub index: ID,

    pub user: AccountId,
    pub class: OrderType,
    pub direction: OrderDirection,

    pub amount: Amount,
    pub hasfill_amount: Amount,
    pub create_time: BlockNumber,
    pub lastupdate_time: BlockNumber,
    pub status: OrderStatus,
    pub reserve_last: Amount, //未被交易 未被回退
    pub fill_index: Vec<ID>,  // 填充历史记录的索引
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Handicap<Price>
where
    Price: Copy,
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
    pub header: BlockHeader,
    pub height: u32,
    pub confirmed: bool,
    pub txid: Vec<H256>,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Compact(u32);

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BlockHeader {
    pub version: u32,
    pub previous_header_hash: substrate_primitives::H256,
    pub merkle_root_hash: substrate_primitives::H256,
    pub time: u32,
    pub bits: Compact,
    pub nonce: u32,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TxInfo {
    pub input_address: btc::Address,
    pub raw_tx: btc::Transaction,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Params {
    max_bits: u32,
    //Compact
    block_max_future: u32,
    max_fork_route_preset: u32,

    target_timespan_seconds: u32,
    target_spacing_seconds: u32,
    retargeting_factor: u32,

    double_spacing_seconds: u32,

    retargeting_interval: u32,
    min_timespan: u32,
    max_timespan: u32,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct UTXOKey {
    pub txid: H256,
    pub index: u32,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct UTXOStatus {
    pub balance: u64,
    pub status: bool,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CandidateTx {
    pub tx: btc::Transaction,
    pub outs: Vec<u32>,
}
