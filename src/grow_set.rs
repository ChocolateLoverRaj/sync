use std::collections::HashSet;

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

impl ViewCrdt for GrowSetCrdt {
    fn view<'a>(
        state: &'a crate::state::State<Self>,
    ) -> impl Into<iced::Element<'a, crate::state::Message<Self>, iced::Theme, iced::Renderer>>
    {
        column![
            column(state.value().into_iter().map(|string| text(string).into())),
            text_input("add a string", "String value")
                .on_input(|a| state.commit(a))
                .on_submit(state.commit("Hello".into()))
        ]
    }
}
