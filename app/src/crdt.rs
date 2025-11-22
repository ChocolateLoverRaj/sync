use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

pub trait Crdt: 'static + Clone + Copy {
    type CommitData: Debug
        + Clone
        + Hash
        + PartialEq
        + Eq
        + Send
        + Sync
        + Serialize
        + for<'a> Deserialize<'a>;
    type Value;

    fn compute<'a, T: Iterator<Item = &'a Self::CommitData>>(iter: T) -> Self::Value;
}
