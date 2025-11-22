use iced::{
    Alignment::Center,
    Renderer, Theme,
    widget::{button, column, text},
};

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct CounterCrdt;
impl Crdt for CounterCrdt {
    type CommitData = i64;
    type Value = i64;

    fn compute<'a, T: Iterator<Item = &'a Self::CommitData>>(iter: T) -> Self::Value {
        iter.sum()
    }
}

impl ViewCrdt for CounterCrdt {
    fn view<'a>(
        state: &'a State<Self>,
    ) -> impl Into<iced::Element<'a, Message<Self>, Theme, Renderer>> {
        column![
            button("Increment").on_press(state.commit(1)),
            text(state.value()).size(50),
            button("Decrement").on_press(state.commit(-1))
        ]
        .padding(20)
        .align_x(Center)
    }
}
