use iced::widget::{Column, button, column, scrollable, text};
use iced::{Element, Renderer, Theme};

use crate::commit::*;
use crate::crdt::*;
use crate::grow_set::*;
use crate::state::*;
use crate::state::*;
use crate::view_crdt::*;

mod commit;
mod counter;
mod crdt;
mod file_storage;
mod grow_set;
mod state;
mod view_crdt;

const SAVE_FILE: &str = "save.ron";
const SAVE_FILE_TEMP: &str = ".save.ron";

pub fn main() -> iced::Result {
    iced::application("Sync Demo", State::update, view).run_with(move || {
        let mut state = State::<GrowSetCrdt>::new(Paths {
            file: SAVE_FILE.into(),
            temp_file: SAVE_FILE_TEMP.into(),
        });
        let task = state.update(state.load());
        (state, task)
    })
}

fn view<T: ViewCrdt>(state: &State<T>) -> impl Into<Element<'_, Message<T>>> {
    scrollable(column![
        T::view(state).into(),
        text(format!("Number of commits: {}", state.commits_len())),
        match state.load_status() {
            LoadStatus::NotLoaded(None) =>
                Element::from(text("Data not loaded from file for some reason")),
            LoadStatus::NotLoaded(Some(load_error)) => column![
                text(format!("Error loading data from file: {load_error:?}")),
                button("Retry").on_press(state.load())
            ]
            .into(),
            LoadStatus::Loading => text("Loading data from file").into(),
            LoadStatus::Loaded => text("Loaded data from file").into(),
        },
        match state.save_status() {
            SaveStatus::NotSaving(None) => Element::from(text(
                "not saving. changes will be automatically saved to file"
            )),
            SaveStatus::NotSaving(Some(save_error)) => column![
                text(format!("Error saving data to file: {save_error:?}")),
                button("Retry").on_press(state.save())
            ]
            .into(),
            SaveStatus::Saving(_) => text("saving changes to file").into(),
            SaveStatus::WaitingForLoad =>
                text("waiting to load from file before saving latest changes to file").into(),
        },
    ])
}
