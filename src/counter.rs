use iced::{
    Alignment::Center,
    Renderer, Theme,
    application::View,
    widget::{button, column, text},
};

use crate::{
    commit::{Crdt, ViewCrdt},
    state::{Message, State},
};

#[derive(Debug, Clone, Copy)]
pub struct CounterCrdt;
impl Crdt for CounterCrdt {
    type CommitData = i64;
    type Value = i64;

    fn compute<T: Iterator<Item = Self::CommitData>>(iter: T) -> Self::Value {
        iter.sum()
    }
}

impl ViewCrdt for CounterCrdt {
    fn view<'a>(
        state: &'a State<Self>,
    ) -> impl Into<iced::Element<'a, Message<Self>, Theme, Renderer>> {
        column![
            button("Increment").on_press(state.change(1)),
            text(state.value()).size(50),
            button("Decrement").on_press(state.change(-1))
        ]
        .padding(20)
        .align_x(Center)
    }
}
