use iced::Element;
use iced::Task;
use iced::widget::{button, column, scrollable, text};

use crate::commit::*;
use crate::counter::*;
use crate::crdt::*;
use crate::grow_set::*;
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
    iced::application("Sync Demo", App::update, App::view).run_with(move || {
        let mut state = State::new(Paths {
            file: SAVE_FILE.into(),
            temp_file: SAVE_FILE_TEMP.into(),
        });
        let task = state.update(state.load());
        (
            App {
                view_crdt: ViewGrowSetCrdt::default(),
                state,
            },
            task.map(Message::State),
        )
    })
}

#[derive(Debug, Clone)]
enum Message<T: ViewCrdt> {
    ViewCrdt(ViewCrdtMessage<T::Crdt, T::Message>),
    State(crate::state::Message<T::Crdt>),
}

struct App<T: ViewCrdt> {
    view_crdt: T,
    state: State<T::Crdt>,
}

impl<T: ViewCrdt + 'static> App<T> {
    pub fn update(&mut self, message: Message<T>) -> Task<Message<T>> {
        match message {
            Message::ViewCrdt(ViewCrdtMessage::Commit(commit)) => self
                .state
                .update(self.state.commit(commit))
                .map(Message::State),
            Message::ViewCrdt(ViewCrdtMessage::Custom(message)) => {
                self.view_crdt.update(message).map(Message::ViewCrdt)
            }
            Message::State(message) => self.state.update(message).map(Message::State),
        }
    }

    pub fn view(&self) -> impl Into<Element<'_, Message<T>>> {
        scrollable(column![
            self.view_crdt
                .view(self.state.value())
                .into()
                .map(Message::ViewCrdt),
            text(format!("Number of commits: {}", self.state.commits_len())),
            match self.state.load_status() {
                LoadStatus::NotLoaded(None) =>
                    Element::from(text("Data not loaded from file for some reason")),
                LoadStatus::NotLoaded(Some(load_error)) => column![
                    text(format!("Error loading data from file: {load_error:?}")),
                    button("Retry").on_press(Message::State(self.state.load()))
                ]
                .into(),
                LoadStatus::Loading => text("Loading data from file").into(),
                LoadStatus::Loaded => text("Loaded data from file").into(),
            },
            match self.state.save_status() {
                SaveStatus::NotSaving(None) => Element::from(text(
                    "not saving. changes will be automatically saved to file"
                )),
                SaveStatus::NotSaving(Some(save_error)) => column![
                    text(format!("Error saving data to file: {save_error:?}")),
                    button("Retry").on_press(Message::State(self.state.save()))
                ]
                .into(),
                SaveStatus::Saving(_) => text("saving changes to file").into(),
                SaveStatus::WaitingForLoad =>
                    text("waiting to load from file before saving latest changes to file").into(),
            },
        ])
    }
}
