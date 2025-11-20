use std::collections::HashSet;
use std::ops::Deref;

use iced::widget::{Column, button, column, text};
use iced::{Center, Task};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::file_storage::{
    FileStorageMessage, ReadResponse, WriteRequest, WriteResponse, subscription,
};

mod file_storage;

const SAVE_FILE: &str = "save.ron";

pub fn main() -> iced::Result {
    let (read_tx, read_rx) = tokio::sync::watch::channel(None);
    let initial_read_request = Some(1);
    read_tx.send_replace(initial_read_request);
    let (write_tx, write_rx) = tokio::sync::watch::channel(None);

    iced::application("Sync Demo", Counter::update, Counter::view)
        // .subscription(|_counter| {
        //     Subscription::run(|| {
        //         futures::stream::once(async {
        //             sleep(Duration::from_secs(2)).await;
        //             let mut string = String::new();
        //             OpenOptions::new()
        //                 .create(true)
        //                 .read(true)
        //                 .write(true)
        //                 .open(SAVE_FILE)
        //                 .await
        //                 .unwrap()
        //                 .read_to_string(&mut string)
        //                 .await
        //                 .unwrap();
        //             let commits = if string.is_empty() {
        //                 Default::default()
        //             } else {
        //                 ron::from_str(&string).unwrap()
        //             };
        //             Message::LoadCommits(commits)
        //         })
        //     })
        // })
        // .subscription(|_| Subscription::subscription(SAVE_FILE, rx))
        .run_with(move || {
            (
                Counter {
                    commits: Default::default(),
                    read_sender: read_tx,
                    last_read_request: initial_read_request,
                    last_read_response: None,
                    write_sender: write_tx,
                    last_write_request: None,
                    last_write_response: None,
                },
                Task::run(
                    subscription(SAVE_FILE, read_rx, write_rx),
                    Message::FileStorage,
                ),
            )
        })
}

#[derive(Debug)]
struct Counter {
    commits: HashSet<Commit<i64>>,
    read_sender: tokio::sync::watch::Sender<Option<usize>>,
    last_read_request: Option<usize>,
    last_read_response: Option<ReadResponse>,
    write_sender: tokio::sync::watch::Sender<Option<WriteRequest>>,
    last_write_request: Option<usize>,
    last_write_response: Option<WriteResponse>,
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
struct Commit<T> {
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

#[derive(Debug, Clone)]
enum Message {
    Update(i64),
    FileStorage(FileStorageMessage),
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Update(change) => {
                self.commits.insert(Commit::new(change));
                let request_id = self.last_write_request.unwrap_or_default() + 1;
                self.last_write_request = Some(request_id);
                self.write_sender.send_replace(Some(WriteRequest {
                    id: request_id,
                    contents: ron::ser::to_string_pretty(&self.commits, Default::default())
                        .unwrap(),
                }));
            }
            Message::FileStorage(response) => match response {
                FileStorageMessage::Read(response) => {
                    if let Ok(string) = response.result.deref() {
                        self.commits
                            .extend(ron::from_str::<HashSet<_>>(string).unwrap());
                    }
                    self.last_read_response = Some(response);
                }
                FileStorageMessage::Write(response) => {
                    self.last_write_response = Some(response);
                }
            },
        }
    }

    fn view(&self) -> Column<Message> {
        let number_of_commits = self.commits.len();
        column![
            button("Increment").on_press(Message::Update(1)),
            text(
                self.commits
                    .iter()
                    .map(|commit| *commit.data())
                    .sum::<i64>()
            )
            .size(50),
            button("Decrement").on_press(Message::Update(-1)),
            text(format!("Number of commits: {number_of_commits}")),
            text(if let Some(last_read_request_id) = self.last_read_request {
                match &self.last_read_response {
                    None => "loading from file",
                    Some(response) => {
                        if response.id == last_read_request_id {
                            match response.result.deref() {
                                Ok(_) => "successfully loaded from file",
                                Err(_) => "error loading from file",
                            }
                        } else {
                            "loading from file"
                        }
                    }
                }
            } else {
                "not loading from file for some reason"
            }),
            text(
                if let Some(last_write_request_id) = self.last_write_request {
                    match &self.last_write_response {
                        None => "saving changes to file",
                        Some(response) => {
                            if response.id == last_write_request_id {
                                match response.result.deref() {
                                    Ok(()) => "successfully saved changes to file",
                                    Err(_) => "error saving changes to file",
                                }
                            } else {
                                "saving changes to file"
                            }
                        }
                    }
                } else {
                    "no changes to save to file"
                }
            ),
            text(format!("{:?}", self.last_write_response))
        ]
        .padding(20)
        .align_x(Center)
    }
}
