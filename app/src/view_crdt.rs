use std::fmt::Debug;

use crdt::*;
use iced::Task;

use crate::*;

#[derive(Debug, Clone)]
pub enum ViewCrdtMessage<T: Crdt, CustomMessage> {
    Commit(T::CommitData),
    Custom(CustomMessage),
}

pub trait ViewCrdt: Default + Clone {
    type Crdt: Crdt;
    type Message: Debug + Clone + Send + Sync + 'static;

    fn update(
        &mut self,
        message: Self::Message,
    ) -> Task<ViewCrdtMessage<Self::Crdt, Self::Message>>;
    fn view(
        &self,
        value: <Self::Crdt as Crdt>::Value,
    ) -> impl Into<Element<'_, ViewCrdtMessage<Self::Crdt, Self::Message>>>;
}
