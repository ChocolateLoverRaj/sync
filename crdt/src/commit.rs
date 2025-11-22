use std::hash::Hash;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Commit<T> {
    id: Uuid,
    data: T,
}

impl<T> Commit<T> {
    pub fn new(data: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }
}
