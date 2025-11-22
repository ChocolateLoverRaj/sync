use std::collections::{HashMap, HashSet};

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct GrowSetCrdt;
impl Crdt for GrowSetCrdt {
    type CommitData = String;
    type Value = HashSet<String>;

    fn compute<'a, T: Iterator<Item = &'a Self::CommitData>>(iter: T) -> Self::Value {
        iter.cloned().collect()
    }

    fn merge<T: IntoIterator<Item = Commit<Self::CommitData>>>(
        iter: T,
    ) -> impl IntoIterator<Item = Commit<Self::CommitData>> {
        iter.into_iter()
            .map(|commit| (commit.data().clone(), commit))
            .collect::<HashMap<_, _>>()
            .into_values()
    }
}
