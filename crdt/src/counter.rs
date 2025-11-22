use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct CounterCrdt;
impl Crdt for CounterCrdt {
    type CommitData = i64;
    type Value = i64;

    fn compute<'a, T: Iterator<Item = &'a Self::CommitData>>(iter: T) -> Self::Value {
        iter.sum()
    }

    fn merge<T: IntoIterator<Item = Commit<Self::CommitData>>>(
        iter: T,
    ) -> impl IntoIterator<Item = Commit<Self::CommitData>> {
        std::iter::once(Commit::new(
            iter.into_iter().map(|commit| commit.into_data()).sum(),
        ))
    }
}
