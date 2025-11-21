use std::hash::Hash;

use iced::{Element, Renderer, Theme, application::View};
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

pub trait Crdt: 'static + Clone + Copy {
    type CommitData: Clone
        + Copy
        + Hash
        + PartialEq
        + Eq
        + Send
        + Sync
        + Serialize
        + for<'a> Deserialize<'a>;
    type Value;

    fn compute<T: Iterator<Item = Self::CommitData>>(iter: T) -> Self::Value;
}

pub trait ViewCrdt: Crdt {
    fn view<'a>(state: &'a State<Self>) -> impl Into<Element<'a, Message<Self>, Theme, Renderer>>;
}
