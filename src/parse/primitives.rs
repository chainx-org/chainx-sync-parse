use parity_codec::{Codec, Decode, Encode};
use serde::{Deserialize, Serialize};

use sr_primitives::traits::Verify;

use crate::types::{btc, Bytes, NodeT};

// ================================================================================================
// Substrate primitives.
// ================================================================================================

pub use substrate_primitives::H256;

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

#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum NetworkType {
    Mainnet,
    Testnet,
}

impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Testnet
    }
}

/// 44 for Mainnet, 42 for Testnet
pub type AddressType = u32;

// ============================================================================
// xaccounts types.
// ============================================================================

pub type Name = XString;
pub type URL = XString;

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct IntentionProps<SessionKey>
where
    SessionKey: Clone + Default + Codec,
{
    pub url: URL,
    pub is_active: bool,
    pub about: XString,
    pub session_key: Option<SessionKey>,
}

// ============================================================================
// xfee/manager types.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SwitchStore {
    pub global: bool,
    pub spot: bool,
    pub xbtc: bool,
    pub sdot: bool,
}

// ============================================================================
// xassets/assets types.
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
    ReservedCurrency,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Free
    }
}

// ============================================================================
// xassets/records types.
// ============================================================================

pub type AddrStr = XString;
pub type Memo = XString;

/// state machine for state is:
/// Applying(lock token) => Processing(can't cancel) =>
///        destroy token => NormalFinish|RootFinish (final state)
///        release token => NormalCancel(can from Applying directly)|RootCancel (final state)
#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum ApplicationState {
    Applying,
    Processing,
    NormalFinish,
    RootFinish,
    NormalCancel,
    RootCancel,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState::Applying
    }
}

/// application for withdrawal
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Application<AccountId, Balance, BlockNumber>
where
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub id: u32,
    pub state: ApplicationState,
    pub applicant: AccountId,
    pub token: Token,
    pub balance: Balance,
    pub addr: AddrStr,
    pub ext: Memo,
    pub height: BlockNumber,
}

impl<AccountId, Balance, BlockNumber> NodeT for Application<AccountId, Balance, BlockNumber>
where
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    type Index = u32;
    fn index(&self) -> Self::Index {
        self.id
    }
}

// ============================================================================
// xmining/staking types.
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
// xmining/tokens types.
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
// xmultisig types.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct AddrInfo<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub addr_type: AddrType,
    pub required_num: u32,
    pub owner_list: Vec<(AccountId, bool)>,
}

// ============================================================================
// xdex/spot types.
// ============================================================================

pub type Price = Balance;

pub type OrderIndex = u64;
pub type TradeHistoryIndex = u64;
pub type TradingPairIndex = u32;

/// PCX/BTC, base currency / quote currency
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CurrencyPair(pub Token, pub Token);

/// PCX/BTC = pip, a.k.a, percentage in point. Also called exchange rate.
/// tick precision for BTC
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TradingPair {
    pub id: TradingPairIndex,
    pub currency_pair: CurrencyPair,
    pub precision: u32,
    pub unit_precision: u32,
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
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
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

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct OrderProperty<
    PairIndex: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
>(
    AccountId,
    PairIndex,
    Side,
    Balance,
    Price,
    OrderIndex,
    OrderType,
    BlockNumber,
);

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Order<PairIndex, AccountId, Balance, Price, BlockNumber>
where
    PairIndex: Clone + Default + Codec,
    AccountId: Clone + Default + Codec,
    Balance: Copy + Default + Codec,
    Price: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub props: OrderProperty<PairIndex, AccountId, Balance, Price, BlockNumber>,

    pub status: OrderStatus,
    pub remaining: Balance,
    pub executed_indices: Vec<TradeHistoryIndex>, // indices of transaction record
    pub already_filled: Balance,
    pub last_update_at: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Handicap<Price>
where
    Price: Copy + Default + Codec,
{
    pub highest_bid: Price,
    pub lowest_offer: Price,
}

// ============================================================================
// xbridge/bitcoin types.
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
    HotAndCold,
    TrusteeTransition,
    Irrelevance,
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
    pub height: u32,
    pub done: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
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
pub struct WithdrawalProposal<AccountId>
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

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeAddrInfo {
    pub addr: btc::Address,
    pub redeem_script: Bytes,
}

impl IntoVecu8 for TrusteeAddrInfo {
    fn into_vecu8(self) -> Vec<u8> {
        parity_codec::Encode::encode(&self)
    }
    fn from_vecu8(src: &[u8]) -> Option<Self> {
        let mut src = src;
        parity_codec::Decode::decode(&mut src)
    }
}

// ============================================================================
// xbridge/sdot types.
// ============================================================================

pub type EthereumAddress = substrate_primitives::H160;

// ============================================================================
// xbridge/features types.
// ============================================================================

pub trait IntoVecu8 {
    fn into_vecu8(self) -> Vec<u8>;
    fn from_vecu8(src: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

impl IntoVecu8 for Vec<u8> {
    fn into_vecu8(self) -> Vec<u8> {
        self
    }
    fn from_vecu8(src: &[u8]) -> Option<Self> {
        Some(src.to_vec())
    }
}

impl IntoVecu8 for [u8; 20] {
    fn into_vecu8(self) -> Vec<u8> {
        self.to_vec()
    }

    fn from_vecu8(src: &[u8]) -> Option<Self> {
        if src.len() != 20 {
            return None;
        }
        let mut a: [u8; 20] = Default::default();
        let len = a.len();
        a.copy_from_slice(&src[..len]);
        Some(a)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeInfoConfig {
    pub min_trustee_count: u32,
    pub max_trustee_count: u32,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeIntentionProps<TrusteeEntity>
where
    TrusteeEntity: IntoVecu8,
{
    pub about: XString,
    pub hot_entity: TrusteeEntity,
    pub cold_entity: TrusteeEntity,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TrusteeSessionInfo<AccountId, TrusteeAddress>
where
    AccountId: Clone + Default + Codec,
    TrusteeAddress: IntoVecu8,
{
    pub trustee_list: Vec<AccountId>,
    pub hot_address: TrusteeAddress,
    pub cold_address: TrusteeAddress,
}

pub type BitcoinTrusteeType = btc::Public;

impl IntoVecu8 for BitcoinTrusteeType {
    fn into_vecu8(self) -> Vec<u8> {
        (&self).to_vec()
    }

    fn from_vecu8(src: &[u8]) -> Option<Self> {
        Self::from_slice(src).ok()
    }
}

pub type BitcoinTrusteeAddrInfo = TrusteeAddrInfo;
pub type BitcoinTrusteeIntentionProps = TrusteeIntentionProps<BitcoinTrusteeType>;
pub type BitcoinTrusteeSessionInfo<AccountId> =
    TrusteeSessionInfo<AccountId, BitcoinTrusteeAddrInfo>;
