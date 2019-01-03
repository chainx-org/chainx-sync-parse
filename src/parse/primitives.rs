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

pub type Moment = u64;

/// The balance of an account.
/// u64 for chainx token and all assets type, if the asset is not suit for u64, choose a suitable precision
pub type Balance = u64;

/// Cert immutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct CertImmutableProps<BlockNumber: Default> {
    pub issued_at: BlockNumber,
    pub frozen_duration: u32,
}

/// Intention Immutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct IntentionImmutableProps {
    pub name: Vec<u8>,
    pub activator: Vec<u8>,
    pub initial_shares: u32,
}

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct IntentionProps {
    pub url: Vec<u8>,
    pub is_active: bool,
}

pub type Token = Vec<u8>;
pub type Desc = Vec<u8>;
pub type Precision = u16;

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Asset {
    token: Token,
    chain: Chain,
    precision: Precision,
    desc: Desc,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum Action {
    Deposit(DepositState),
    Withdrawal(WithdrawalState),
}

impl Default for Action {
    /// default not use for Action enum, it's just for the trait
    fn default() -> Self {
        Action::Deposit(DepositState::default())
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Record<Token, Balance, BlockNumber>
where
    Token: Clone + Default,
    Balance: Copy + Default,
    BlockNumber: Copy + Default,
{
    action: Action,
    token: Token,
    balance: Balance,
    init_blocknum: BlockNumber,
    txid: Vec<u8>,
    addr: Vec<u8>,
    ext: Vec<u8>,
}

/// Intention mutable properties
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct IntentionProfs<Balance: Default, BlockNumber: Default> {
    pub jackpot: Balance,
    pub total_nomination: Balance,
    pub last_total_vote_weight: u64,
    pub last_total_vote_weight_update: BlockNumber,
}

/// Nomination record of one of the nominator's nominations.
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct NominationRecord<Balance: Default, BlockNumber: Default> {
    pub nomination: Balance,
    pub last_vote_weight: u64,
    pub last_vote_weight_update: BlockNumber,
    pub revocations: Vec<(BlockNumber, Balance)>,
}

/// This module only tracks the vote weight related changes.
/// All the amount related has been taken care by assets module.
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct PseduIntentionVoteWeight<Balance: Default, BlockNumber: Default> {
    pub jackpot: Balance,
    pub last_total_deposit_weight: u64,
    pub last_total_deposit_weight_update: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct DepositVoteWeight<BlockNumber: Default> {
    pub last_deposit_weight: u64,
    pub last_deposit_weight_update: BlockNumber,
}

pub type Amount = u128;
pub type Price = u128;
pub type BidId = u128;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Bid<Amount, Price>
where
    Amount: Copy + Default,
    Price: Copy + Default,
{
    nodeid: u128,
    price: Price,
    sum: Amount,
    list: Vec<BidId>,
}

//pub type BidT = Bid<Amount, Price>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct BidDetail<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone + Default,
    AccountId: Clone + Default,
    Amount: Copy + Default,
    Price: Copy + Default,
    BlockNumber: Copy + Default,
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

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum OrderType {
    Buy,
    Sell,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Buy
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub enum CommandType {
    Match,
    Cancel,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::Match
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct OrderPair {
    pub first: Token,
    pub second: Token,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct OrderPairDetail {
    pub precision: u32, //价格精度
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
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
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
pub struct Order<Pair, AccountId, Amount, Price, BlockNumber>
where
    Pair: Clone + Default,
    AccountId: Clone + Default,
    Amount: Copy + Default,
    Price: Copy + Default,
    BlockNumber: Copy + Default,
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

//#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct BestHeader {
//    /// Height/number of the best block (genesis block has zero height)
//    pub number: u32,
//    /// Hash of the best block
//    pub hash: H256,
//}
//
//#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, Default)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct Params {
//    max_bits: u32,
//    //Compact
//    block_max_future: u32,
//    max_fork_route_preset: u32,
//
//    target_timespan_seconds: u32,
//    target_spacing_seconds: u32,
//    retargeting_factor: u32,
//
//    double_spacing_seconds: u32,
//
//    retargeting_interval: u32,
//    min_timespan: u32,
//    max_timespan: u32,
//}
//
//#[derive(PartialEq, Eq, Clone, Encode, Decode, Default)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct UTXO {
//    pub txid: H256,
//    pub index: u32,
//    pub balance: u64,
//    pub is_spent: bool,
//}
//
//#[derive(PartialEq, Clone, Encode, Decode, Default)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct BindInfo<AccountId: Parameter + Ord + Default> {
//    pub account: AccountId,
//    pub channel: Vec<u8>,
//}
//
//#[derive(PartialEq, Clone, Encode, Decode, Default, Serialize, Deserialize, Debug)]
////#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct DepositInfo<AccountId: Parameter + Ord + Default> {
//    pub account: AccountId,
//    pub btc_balance: u64,
//    pub tx_hash: H256,
//    pub block_hash: H256,
//    pub channel: Vec<u8>,
//}
//
//#[derive(PartialEq, Clone, Encode, Decode, Default)]
//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
//pub struct DepositHistInfo {
//    pub btc_balance: u64,
//    pub tx_hash: H256,
//    pub block_hash: H256,
//    pub channel: Vec<u8>,
//}
