use iced::Center;
use iced::widget::{Column, button, column, row, text};

use crate::state::*;

mod commit;
mod file_storage;
mod state;

const SAVE_FILE: &str = "save.ron";
const SAVE_FILE_TEMP: &str = ".save.ron";

pub fn main() -> iced::Result {
    iced::application("Sync Demo", State::update, view).run_with(move || {
        let mut state = State::new(Paths {
            file: SAVE_FILE.into(),
            temp_file: SAVE_FILE_TEMP.into(),
        });
        let task = state.update(state.load());
        (state, task)
    })
}

fn view(state: &State) -> Column<'_, Message> {
    column![
        column![
            button("Increment").on_press(state.change(1)),
            text(state.value()).size(50),
            button("Decrement").on_press(state.change(-1))
        ]
        .padding(20)
        .align_x(Center),
        text(format!("Number of commits: {}", state.commits_len())),
        match state.load_status() {
            LoadStatus::NotLoaded(None) => row![text("Data not loaded from file for some reason")],
            LoadStatus::NotLoaded(Some(load_error)) => row![
                text(format!("Error loading data from file: {load_error:?}")),
                button("Retry").on_press(state.load())
            ],
            LoadStatus::Loading => row![text("Loading data from file")],
            LoadStatus::Loaded => row![text("Loaded data from file")],
        },
        match state.save_status() {
            SaveStatus::NotSaving(None) => row![text(
                "not saving. changes will be automatically saved to file"
            )],
            SaveStatus::NotSaving(Some(save_error)) => row![
                text(format!("Error saving data to file: {save_error:?}")),
                button("Retry").on_press(state.save())
            ],
            SaveStatus::Saving(_) => row![text("saving changes to file")],
            SaveStatus::WaitingForLoad => row![text(
                "waiting to load from file before saving latest changes to file"
            )],
        },
    ]
}
