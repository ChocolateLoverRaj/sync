use std::hash::Hash;

use iced::{Element, Renderer, Theme};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::{Message, State};

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
}
