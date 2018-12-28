use sr_primitives::generic;
use sr_primitives::traits::BlakeTwo256;

/// Signature on candidate's block data by a collator.
pub type CandidateSignature = sr_primitives::Ed25519Signature;

/// The Ed25519 pub key of an session that belongs to an authority of the relay chain. This is
/// exactly equivalent to what the substrate calls an "authority".
pub type SessionKey = substrate_primitives::AuthorityId;

/// A hash of some data used by the relay chain.
pub type Hash = substrate_primitives::H256;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, SessionKey>>;

/// Opaque, encoded, unchecked extrinsic.
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with = "bytes"))] pub Vec<u8>);

/// A "future-proof" block type for Polkadot. This will be resilient to upgrades in transaction
/// format, because it doesn't attempt to decode extrinsics.
///
/// Specialized code needs to link to (at least one version of) the runtime directly
/// in order to handle the extrinsics within.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// An index to a block.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
/// TODO: switch to u32
pub type BlockNumber = u64;

/// Alias to Ed25519 pubkey that identifies an account on the relay chain.
pub type AccountId = substrate_primitives::H256;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u64;

/// Indentifier for a chain. 32-bit should be plenty.
pub type ChainId = u32;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Index = u64;

pub type Signature = sr_primitives::Ed25519Signature;

/// A timestamp: seconds since the unix epoch.
pub type Timestamp = u64;

/// The balance of an account.
/// 128-bits (or 38 significant decimal figures) will allow for 10m currency (10^7) at a resolution
/// to all for one second's worth of an annualised 50% reward be paid to a unit holder (10^11 unit
/// denomination), or 10^18 total atomic units, to grow at 50%/year for 51 years (10^9 multiplier)
/// for an eventual total of 10^27 units (27 significant decimal figures).
/// We round denomination to 10^12 (12 sdf), and leave the other redundancy at the upper end so
/// that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow.
pub type Balance = u64;

/// "generic" block ID for the future-proof block type.
// TODO: parameterize blockid only as necessary.
pub type BlockId = generic::BlockId<Block>;

/// Inherent data to include in a block.
#[derive(Encode, Decode)]
pub struct InherentData {
    /// Current timestamp.
    pub timestamp: Timestamp,
    /// Indices of offline validators.
    pub offline_indices: Vec<u32>,
    /// block producer
    pub block_producer: AccountId,
}

/// Candidate receipt type.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct CandidateReceipt {
    /// chainx account id.
    pub collator: AccountId,
    /// Signature on blake2-256 of the block data by collator.
    pub signature: CandidateSignature,
    /// blake2-256 Hash of block data.
    pub block_data_hash: Hash,
}

impl CandidateReceipt {
    /// Get the blake2_256 hash
    #[cfg(feature = "std")]
    pub fn hash(&self) -> Hash {
        use sr_primitives::traits::{BlakeTwo256, Hash};
        BlakeTwo256::hash_of(self)
    }

    /// Check integrity vs. provided block data.
    pub fn check_signature(&self) -> Result<(), ()> {
        use sr_primitives::traits::Verify;

        if self
            .signature
            .verify(self.block_data_hash.as_ref(), &self.collator)
        {
            Ok(())
        } else {
            Err(())
        }
    }
}

// ChainX ---------------------------------------------------------------------

/// Cert immutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct CertImmutableProps<BlockNumber: Default> {
    pub issued_at: BlockNumber,
    pub frozen_duration: u32,
}

/// Intention Immutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct IntentionImmutableProps {
    pub name: Vec<u8>,
    pub activator: Vec<u8>,
    pub initial_shares: u32,
}

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct IntentionProps {
    pub url: Vec<u8>,
    pub is_active: bool,
}

pub type Token = Vec<u8>;
pub type Desc = Vec<u8>;
pub type Precision = u16;

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum Chain {
    PCX,
    BTC,
    ETH,
}

impl Default for Chain {
    fn default() -> Self {
        Chain::PCX
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Asset {
    token: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum ReservedType {
    Others,
    Staking,
    AssetsWithdrawal,
    DexSpot,
    DexFuture,
}

impl Default for ReservedType {
    fn default() -> Self {
        ReservedType::Others
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum DepositState {
    Invalid,
    Success,
    Failed,
}

impl Default for DepositState {
    fn default() -> Self {
        DepositState::Invalid
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum WithdrawalState {
    Invalid,
    Locking,
    Success,
    Failed,
}

impl Default for WithdrawalState {
    fn default() -> Self {
        WithdrawalState::Invalid
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum Action {
    Deposit(DepositState),
    Withdrawal(WithdrawalState),
}

impl Default for Action {
    /// default not use for Action enum, it's just for the trait
    fn default() -> Self {
        Action::Deposit(Default::default())
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Record<Token, Balance, BlockNumber>
where
    Token: Clone,
    Balance: Copy,
    BlockNumber: Copy,
{
    action: Action,
    token: Token,
    balance: Balance,
    init_blocknum: BlockNumber,
    txid: Vec<u8>,
    addr: Vec<u8>,
    ext: Vec<u8>,
}

pub type Amount = u128;
pub type Price = u128;
pub type BidId = u128;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Bid<Amount, Price>
where
    Amount: Copy,
    Price: Copy,
{
    nodeid: u128,
    price: Price,
    sum: Amount,
    list: Vec<BidId>,
}

pub type BidT = Bid<Amount, Price>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct BidDetail<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone,
    AccountId: Clone,
    Amount: Copy,
    Price: Copy,
    BlockNumber: Copy,
{
    id: BidId,
    pair: Pair,
    order_type: OrderType,
    user: AccountId,
    order_index: u64,
    price: Price,
    amount: Amount,
    time: BlockNumber,
}

pub type BidDetailT = BidDetail<OrderPair, AccountId, Amount, Price, BlockNumber>;

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum OrderType {
    Buy,
    Sell,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Buy
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum CommandType {
    Match,
    Cancel,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::Match
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct OrderPair {
    pub first: Token,
    pub second: Token,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct OrderPairDetail {
    pub precision: u32, //价格精度
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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

pub type Channel = Vec<u8>;

/// 用户的挂单记录 包含了成交历史的index
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Order<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone,
    AccountId: Clone,
    Amount: Copy,
    Price: Copy,
    BlockNumber: Copy,
{
    pair: Pair,
    index: u64,
    class: OrderType,
    user: AccountId,
    amount: Amount,
    channel: Channel,
    hasfill_amount: Amount,
    price: Price,
    create_time: BlockNumber,
    lastupdate_time: BlockNumber,
    status: OrderStatus,
    fill_index: Vec<u128>, // 填充历史记录的索引
    reserve_last: Amount,  //未被交易 未被回退
}

pub type OrderT = Order<OrderPair, AccountId, Amount, Price, BlockNumber>;
