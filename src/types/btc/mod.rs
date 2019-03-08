#![allow(unused_must_use)]

use parity_codec::{Decode, Encode, Input};
use serde_derive::{Deserialize, Serialize};

mod compact_integer;
mod impls;
mod io;
mod reader;
mod stream;

use super::Bytes;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Compact(u32);

impl Compact {
    pub fn new(u: u32) -> Self {
        Compact(u)
    }
}

impl From<u32> for Compact {
    fn from(u: u32) -> Self {
        Compact(u)
    }
}

impl From<Compact> for u32 {
    fn from(c: Compact) -> Self {
        c.0
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BlockHeader {
    pub version: u32,
    pub previous_header_hash: substrate_primitives::H256,
    pub merkle_root_hash: substrate_primitives::H256,
    pub time: u32,
    pub bits: Compact,
    pub nonce: u32,
}

impl Encode for BlockHeader {
    fn encode(&self) -> Vec<u8> {
        let value = stream::serialize::<BlockHeader>(&self);
        value.encode()
    }
}

impl Decode for BlockHeader {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let value: Vec<u8> = Decode::decode(input).unwrap();
        if let Ok(header) = reader::deserialize(reader::Reader::new(&value)) {
            Some(header)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum Type {
    /// Pay to PubKey Hash
    /// Common P2PKH which begin with the number 1, eg: 1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2.
    /// https://bitcoin.org/en/glossary/p2pkh-address
    P2PKH,
    /// Pay to Script Hash
    /// Newer P2SH type starting with the number 3, eg: 3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy.
    /// https://bitcoin.org/en/glossary/p2sh-address
    P2SH,
}

impl Default for Type {
    fn default() -> Self {
        Type::P2PKH
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum Network {
    Mainnet,
    Testnet,
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

/// `AddressHash` with network identifier and format type
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Address {
    /// The type of the address.
    pub kind: Type,
    /// The network of the address.
    pub network: Network,
    /// Public key hash.
    pub hash: substrate_primitives::H160,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn has_witness(&self) -> bool {
        self.inputs.iter().any(TransactionInput::has_witness)
    }
}

impl Encode for Transaction {
    fn encode(&self) -> Vec<u8> {
        let value = stream::serialize::<Transaction>(&self);
        value.encode()
    }
}

impl Decode for Transaction {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        let value: Vec<u8> = Decode::decode(input).unwrap();
        if let Ok(tx) = reader::deserialize(reader::Reader::new(&value)) {
            Some(tx)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Bytes,
    pub sequence: u32,
    pub script_witness: Vec<Bytes>,
}

impl TransactionInput {
    pub fn has_witness(&self) -> bool {
        !self.script_witness.is_empty()
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct OutPoint {
    pub hash: substrate_primitives::H256,
    pub index: u32,
}

#[derive(PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Bytes,
}
