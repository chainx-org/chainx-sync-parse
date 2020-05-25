#![allow(clippy::type_repetition_in_bounds)]

use parity_codec::{Codec, Decode, Encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{btc, Bytes, NodeT};

// ================================================================================================
// Substrate primitives.
// ================================================================================================

pub use primitive_types::{H256, H512};

/// A public key.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
#[derive(Encode, Decode)]
pub struct Public(pub [u8; 32]);

impl AsRef<[u8]> for Public {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl From<Public> for H256 {
    fn from(x: Public) -> Self {
        x.0.into()
    }
}

impl Public {
    /// A new instance from an H256.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    fn from_h256(x: H256) -> Self {
        Public(x.into())
    }
}

impl core::fmt::Display for Public {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", to_ss58check(self))
    }
}

impl core::fmt::Debug for Public {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = to_ss58check(self);
        write!(f, "{} ({}...)", hex::encode(&self.0), &s[0..8])
    }
}

impl Serialize for Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO wait until issue https://github.com/paritytech/substrate/issues/2064 fix
        // serializer.serialize_str(&self.to_ss58check())
        let h256: H256 = self.clone().into();
        h256.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO wait until issue https://github.com/paritytech/substrate/issues/2064 fix
        // Public::from_ss58check(&String::deserialize(deserializer)?)
        // 	.map_err(|e| de::Error::custom(format!("{:?}", e)))
        let h256: H256 = H256::deserialize(deserializer)?;
        Ok(Public::from_h256(h256))
    }
}

const PREFIX: &[u8] = b"SS58PRE";

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}

fn to_ss58check(data: impl AsRef<[u8]>) -> String {
    // let mut v = vec![42u8];
    let mut v = vec![44u8];
    v.extend(data.as_ref());
    let r = ss58hash(&v);
    v.extend(&r.as_bytes()[0..2]);
    bs58::encode(v).into_string()
}

// ================================================================================================
// ChainX primitives.
// ================================================================================================

//use sr_primitives::traits::Verify;
//type AuthoritySignature = substrate_primitives::ed25519::Signature;
//type AuthorityId = <AuthoritySignature as Verify>::Signer;
//type AuthorityId = substrate_primitives::ed25519::Public;
type AuthorityId = Public;

//type Signature = substrate_primitives::ed25519::Signature;
//pub type AccountId = <Signature as Verify>::Signer;
//pub type AccountId = substrate_primitives::ed25519::Public;
pub type AccountId = Public;

pub type SessionKey = AuthorityId;

pub type BlockNumber = u64;

pub type AccountIndex = u32;

pub type Index = u64;

pub type Timestamp = u64;

pub type Balance = u64;

pub type XString = String;

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct IntentionProps<SessionKey, BlockNumber>
where
    SessionKey: Clone + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub url: URL,
    pub is_active: bool,
    pub about: XString,
    pub session_key: Option<SessionKey>,
    pub registered_at: BlockNumber,
    pub last_inactive_since: BlockNumber,
}

// ============================================================================
// xfee/manager types.
// ============================================================================

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum CallSwitcher {
    Global,
    Spot,
    XBTC,
    XBTCLockup,
    SDOT,
    Contracts,
}

impl Default for CallSwitcher {
    fn default() -> Self {
        CallSwitcher::Global
    }
}

// ============================================================================
// xassets/assets types.
// ============================================================================

pub type Token = XString;
pub type Desc = XString;
pub type Precision = u16;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct Asset {
    token: Token,
    token_name: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum AssetType {
    Free,
    ReservedStaking,
    ReservedStakingRevocation,
    ReservedWithdrawal,
    ReservedDexSpot,
    ReservedDexFuture,
    ReservedCurrency,
    GasPayment,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Free
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum AssetLimit {
    CanMove,
    CanTransfer,
    CanDeposit,
    CanWithdraw,
    CanDestroyWithdrawal,
    CanDestroyFree,
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
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct IntentionProfs<Balance, BlockNumber>
where
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub total_nomination: Balance,
    pub last_total_vote_weight: u64,
    pub last_total_vote_weight_update: BlockNumber,
}

/// Intention mutable properties v1
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct IntentionProfsV1<Balance, BlockNumber>
where
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub total_nomination: Balance,
    pub last_total_vote_weight: u128,
    pub last_total_vote_weight_update: BlockNumber,
}

/// Nomination record of one of the nominator's nominations.
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

/// Nomination record v1 of one of the nominator's nominations.
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct NominationRecordV1<Balance, BlockNumber>
where
    Balance: Copy + Default + Codec,
    BlockNumber: Copy + Default + Codec,
{
    pub nomination: Balance,
    pub last_vote_weight: u128,
    pub last_vote_weight_update: BlockNumber,
    pub revocations: Vec<(BlockNumber, Balance)>,
}

// ============================================================================
// xmining/tokens types.
// ============================================================================

/// This module only tracks the vote weight related changes.
/// All the amount related has been taken care by assets module.
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct PseduIntentionVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct PseduIntentionVoteWeightV1<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_total_deposit_weight: u128,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct DepositVoteWeight<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct DepositVoteWeightV1<BlockNumber>
where
    BlockNumber: Copy + Default + Codec,
{
    pub last_deposit_weight: u128,
    pub last_deposit_weight_update: BlockNumber,
}

// ============================================================================
// xmultisig types.
// ============================================================================

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum MultiSigPermission {
    ConfirmOnly,
    ConfirmAndPropose,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct AddrInfo<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub addr_type: AddrType,
    pub required_num: u32,
    pub owner_list: Vec<(AccountId, MultiSigPermission)>,
}

// ============================================================================
// xdex/spot types.
// ============================================================================

pub type Price = Balance;

pub type OrderIndex = u64;
pub type TradeHistoryIndex = u64;
pub type TradingPairIndex = u32;

/// PCX/BTC, base currency / quote currency
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct CurrencyPair(pub Token, pub Token);

/// PCX/BTC = pip, a.k.a, percentage in point. Also called exchange rate.
/// tick precision for BTC
#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct TradingPair {
    pub id: TradingPairIndex,
    pub currency_pair: CurrencyPair,
    pub pip_precision: u32,
    pub tick_precision: u32,
    pub online: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct BlockHeaderInfo {
    pub header: btc::BlockHeader,
    pub height: u32,
    pub confirmed: bool,
    pub txid_list: Vec<H256>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum TxType {
    Withdraw,
    Deposit,
    HotAndCold,
    TrusteeTransition,
    Lock,
    Unlock,
    Irrelevance,
}

impl Default for TxType {
    fn default() -> Self {
        TxType::Deposit
    }
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct TxInfo {
    pub raw_tx: btc::Transaction,
    pub tx_type: TxType,
    pub height: u32,
    pub done: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct DepositCache {
    pub txid: H256,
    pub balance: u64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub enum VoteResult {
    Unfinish,
    Finish,
}

impl Default for VoteResult {
    fn default() -> Self {
        VoteResult::Unfinish
    }
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct WithdrawalProposal<AccountId>
where
    AccountId: Clone + Default + Codec,
{
    pub sig_state: VoteResult,
    pub withdrawal_id_list: Vec<u32>,
    pub tx: btc::Transaction,
    pub trustee_list: Vec<(AccountId, bool)>,
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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

pub type EthereumAddress = primitive_types::H160;

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

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct TrusteeInfoConfig {
    pub min_trustee_count: u32,
    pub max_trustee_count: u32,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct TrusteeIntentionProps<TrusteeEntity>
where
    TrusteeEntity: IntoVecu8,
{
    pub about: XString,
    pub hot_entity: TrusteeEntity,
    pub cold_entity: TrusteeEntity,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
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
