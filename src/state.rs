use std::collections::HashSet;

use crate::Commit;

pub struct State {
    commits: HashSet<Commit<i64>>,
}
