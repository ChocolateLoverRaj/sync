use iced::{
    Alignment::Center,
    task::Task,
    widget::{button, column, text},
};

use crate::*;
use crdt::*;

#[derive(Debug, Default, Clone)]
pub struct ViewCounterCrdt;

impl ViewCrdt for ViewCounterCrdt {
    type Crdt = CounterCrdt;
    type Message = ();

    fn update(
        &mut self,
        mesesage: Self::Message,
    ) -> Task<ViewCrdtMessage<Self::Crdt, Self::Message>> {
        let _ = mesesage;
        Task::none()
    }
    fn view(
        &self,
        value: <Self::Crdt as Crdt>::Value,
    ) -> impl Into<Element<'_, ViewCrdtMessage<Self::Crdt, Self::Message>>> {
        column![
            button("Increment").on_press(ViewCrdtMessage::Commit(1)),
            text(value).size(50),
            button("Decrement").on_press(ViewCrdtMessage::Commit(-1))
        ]
        .padding(20)
        .align_x(Center)
    }
}
