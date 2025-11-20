use std::collections::HashSet;
use std::time::Duration;

use iced::widget::{Column, button, column, text};
use iced::{Center, Subscription, futures};
use serde::{Deserialize, Serialize};
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use tokio::time::sleep;
use uuid::Uuid;

mod file_storage;

const SAVE_FILE: &str = "save.ron";

pub fn main() -> iced::Result {
    iced::application("Sync Demo", Counter::update, Counter::view)
        .subscription(|_counter| {
            Subscription::run(|| {
                futures::stream::once(async {
                    sleep(Duration::from_secs(2)).await;
                    let mut string = String::new();
                    OpenOptions::new()
                        .create(true)
                        .read(true)
                        .write(true)
                        .open(SAVE_FILE)
                        .await
                        .unwrap()
                        .read_to_string(&mut string)
                        .await
                        .unwrap();
                    let commits = if string.is_empty() {
                        Default::default()
                    } else {
                        ron::from_str(&string).unwrap()
                    };
                    Message::LoadCommits(commits)
                })
            })
        })
        .run()
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Counter {
    commits: HashSet<Commit<i64>>,
    loaded: bool,
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
    LoadCommits(HashSet<Commit<i64>>),
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Update(change) => {
                self.commits.insert(Commit::new(change));
            }
            Message::LoadCommits(commits) => {
                self.commits.extend(commits);
                self.loaded = true;
            }
        }
        // write(
        //     SAVE_FILE,
        //     ron::ser::to_string_pretty(&self, Default::default()).unwrap(),
        // )
        // .unwrap();
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
            text(if !self.loaded {
                "Loading from file"
            } else {
                "Loaded from file"
            }),
        ]
        .padding(20)
        .align_x(Center)
    }
}
