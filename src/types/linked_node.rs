#![allow(clippy::type_repetition_in_bounds)]

use std::fmt::Debug;

use parity_codec::{Codec, Decode, Encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A type that implements Serialize, DeserializeOwned and Debug when in std environment.
pub trait MaybeSerializeDebug: Serialize + DeserializeOwned + Debug {}
impl<T: Serialize + DeserializeOwned + Debug> MaybeSerializeDebug for T {}

pub trait NodeT {
    type Index: Codec + Clone + Eq + PartialEq + Default + MaybeSerializeDebug;
    fn index(&self) -> Self::Index;
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct Node<T: NodeT> {
    prev: Option<T::Index>,
    next: Option<T::Index>,
    pub data: T,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct NodeIndex<T: NodeT> {
    index: T::Index,
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
#[derive(Encode, Decode)]
#[derive(Serialize, Deserialize)]
pub struct MultiNodeIndex<K, T>
where
    K: Codec + Clone + Eq + PartialEq + Default,
    T: NodeT,
{
    multi_key: K,
    index: T::Index,
}
