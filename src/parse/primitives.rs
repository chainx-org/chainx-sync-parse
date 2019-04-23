use parity_codec::Codec;
use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};

use sr_primitives::traits::Verify;

use crate::types::{btc, Bytes, NodeT};

// ================================================================================================
// Substrate primitives.
// ================================================================================================

pub use substrate_primitives::H256;

//pub use srml_balances::VestingSchedule;
/// Struct to encode the vesting schedule of an individual account.
#[derive(Copy, Clone, PartialEq, Eq, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct VestingSchedule<Balance>
where
    Balance: Copy + Default + Codec,
{
    /// Locked amount at genesis.
    pub offset: Balance,
    /// Amount that gets unlocked every block from genesis.
    pub per_block: Balance,
}

//#[derive(Clone, PartialEq, Eq, Default, Encode, Decode, Debug, Serialize, Deserialize)]
//pub struct BalanceLock<Balance, BlockNumber>
//where
//    Balance: Copy + Default + Codec,
//    BlockNumber: Copy + Default + Codec,
//{
//    pub id: LockIdentifier,
//    pub amount: Balance,
//    pub until: BlockNumber,
//    pub reasons: WithdrawReasons,
//}

// ================================================================================================
// ChainX primitives.
// ================================================================================================

type AuthoritySignature = substrate_primitives::ed25519::Signature;
type AuthorityId = <AuthoritySignature as Verify>::Signer;

type Signature = substrate_primitives::ed25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;

pub type Hash = substrate_primitives::H256;

pub type SessionKey = AuthorityId;

pub type BlockNumber = u64;

pub type AccountIndex = u32;

pub type Index = u64;

pub type Timestamp = u64;

pub type Balance = u64;

pub type XString = String;

// ============================================================================
// xaccounts runtime module definitions.
// ============================================================================

pub type Name = XString;
pub type URL = XString;

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct IntentionProps<SessionKey>
where
    SessionKey: Clone + Default + Codec,
{
    pub url: URL,
    pub is_active: bool,
    pub about: XString,
    pub session_key: Option<SessionKey>,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Encode, Decode, Debug, Serialize, Deserialize)]
pub enum TrusteeEntity {
    Bitcoin(Bytes),
}

impl Default for TrusteeEntity {
    fn default() -> Self {
        TrusteeEntity::Bitcoin(Bytes::default())
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TrusteeIntentionProps {
    pub about: XString,
    pub hot_entity: TrusteeEntity,
    pub cold_entity: TrusteeEntity,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TrusteeSessionInfo<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub trustee_list: Vec<AccountId>,
    pub hot_address: Bytes,
    pub cold_address: Bytes,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TrusteeInfoConfig {
    pub min_trustee_count: u32,
    pub max_trustee_count: u32,
}

// ============================================================================
// xfee - manager runtime module definitions.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize,
)]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct Asset {
    token: Token,
    token_name: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize,
)]
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
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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

impl<AccountId, Balance, Moment> NodeT for Application<AccountId, Balance, Moment>
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
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct PseduIntentionVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct DepositVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}

// ============================================================================
// xmultisig - multisig runtime module definitions.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize)]
pub enum AddrType {
    Normal,
    Root,
    Trustee,
}

impl Default for AddrType {
    fn default() -> Self {
        AddrType::Normal
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct AddrInfo<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub addr_type: AddrType,
    pub required_num: u32,
    pub owner_list: Vec<(AccountId, bool)>,
}

// struct for the status of a pending operation.
//#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
//#[cfg_attr(feature = "std", derive(Debug, Serialize))]
//pub struct PendingState<Proposal> {
//    yet_needed: u32,
//    owners_done: u32,
//    proposal: Box<Proposal>,
//}

// ============================================================================
// xdex - spot runtime module definitions.
// ============================================================================

pub type Price = Balance;
pub type Amount = Balance;

pub type OrderIndex = u64;
pub type TradeHistoryIndex = u64;
pub type TradingPairIndex = u32;

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct CurrencyPair(pub Token, pub Token); // base currency / counter currency

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TradingPair {
    pub id: TradingPairIndex,
    pub currency_pair: CurrencyPair,
    pub precision: u32,      // price precision
    pub unit_precision: u32, // minimum unit precision
    pub online: bool,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize,
)]
pub enum OrderType {
    Limit,
    Market,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize,
)]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize,
)]
pub enum OrderStatus {
    ZeroFill,
    ParitialFill,
    Filled,
    ParitialFillAndCanceled,
    Canceled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::ZeroFill
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct OrderProperty<
    Pair: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Amount: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
>(
    AccountId,
    Pair,
    Side,
    Amount,
    Price,
    OrderIndex,
    OrderType,
    BlockNumber,
);

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct Order<Pair, AccountId, Balance, Price, BlockNumber>
where
    Pair: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub props: OrderProperty<Pair, AccountId, Balance, Price, BlockNumber>,

    pub status: OrderStatus,
    pub remaining: Balance,
    pub executed_indices: Vec<TradeHistoryIndex>, // indices of transaction record
    pub already_filled: Balance,
    pub last_update_at: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct BlockHeaderInfo {
    pub header: btc::BlockHeader,
    pub height: u32,
    pub confirmed: bool,
    pub txid_list: Vec<H256>,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize)]
pub enum TxType {
    Withdraw,
    Deposit,
}

impl Default for TxType {
    fn default() -> Self {
        TxType::Deposit
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TxInfo {
    pub raw_tx: btc::Transaction,
    pub tx_type: TxType,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct DepositCache {
    pub txid: H256,
    pub balance: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Debug, Serialize, Deserialize)]
pub enum VoteResult {
    Unfinish,
    Finish,
}

impl Default for VoteResult {
    fn default() -> Self {
        VoteResult::Unfinish
    }
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct WithdrawalProposal<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub sig_state: VoteResult,
    pub withdrawal_id_list: Vec<u32>,
    pub tx: btc::Transaction,
    pub trustee_list: Vec<(AccountId, bool)>,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct TrusteeScriptInfo {
    pub hot_redeem_script: Bytes,
    pub cold_redeem_script: Bytes,
}

// ============================================================================
// xbridge - sdot runtime module definitions.
// ============================================================================

pub type EthereumAddress = substrate_primitives::H160;
