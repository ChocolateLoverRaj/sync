use std::collections::HashSet;

use iced::Task;
use iced::widget::{column, text, text_input};

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct GrowSetCrdt;
impl Crdt for GrowSetCrdt {
    type CommitData = String;
    type Value = HashSet<String>;

    fn compute<'a, T: Iterator<Item = &'a Self::CommitData>>(iter: T) -> Self::Value {
        iter.cloned().collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ViewGrowSetCrdt {
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Input(String),
    Submit,
}

impl ViewCrdt for ViewGrowSetCrdt {
    type Crdt = GrowSetCrdt;
    type Message = Message;

    fn update(
        &mut self,
        message: Self::Message,
    ) -> Task<ViewCrdtMessage<Self::Crdt, Self::Message>> {
        match message {
            Message::Input(input) => {
                self.input = input;
                Task::none()
            }
            Message::Submit => Task::done(ViewCrdtMessage::Commit(self.input.clone())),
        }
    }

    fn view(
        &self,
        value: <Self::Crdt as Crdt>::Value,
    ) -> impl Into<Element<'_, ViewCrdtMessage<Self::Crdt, Self::Message>>> {
        column![
            column(value.into_iter().map(|string| text(string).into())),
            text_input("add a string", &self.input)
                .on_input(|string| ViewCrdtMessage::Custom(Message::Input(string)))
                .on_submit(ViewCrdtMessage::Custom(Message::Submit))
        ]
    }
}
