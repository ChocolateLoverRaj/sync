use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::Commit;

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

    /// Any modified commits should have a new id
    fn merge<T: IntoIterator<Item = Commit<Self::CommitData>>>(
        iter: T,
    ) -> impl IntoIterator<Item = Commit<Self::CommitData>> {
        iter
    }
}
