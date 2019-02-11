// Copyright 2019 Chainpool.

use parity_codec::Codec;
use parity_codec_derive::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};
use sr_primitives::traits::MaybeSerializeDebug;

pub trait NodeT {
    type Index: Codec + Clone + Eq + PartialEq + Default + MaybeSerializeDebug;
    fn index(&self) -> Self::Index;
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Node<T: NodeT> {
    prev: Option<T::Index>,
    next: Option<T::Index>,
    pub data: T,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct NodeIndex<T: NodeT> {
    index: T::Index,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct MultiNodeIndex<K, T>
where
    K: Codec + Clone + Eq + PartialEq + Default,
    T: NodeT,
{
    multi_key: K,
    index: T::Index,
}
