use std::collections::HashSet;

use iced::Center;
use iced::widget::{Column, button, column, text};

pub fn main() -> iced::Result {
    iced::run("Sync Demo", Counter::update, Counter::view)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CommitId {
    user_id: u64,
    counter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Commit {
    id: CommitId,
    change: i64,
}

#[derive(Default)]
struct Counter {
    user_id: u64,
    commits: HashSet<Commit>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Change(i64),
    Import,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Change(change) => {
                self.commits.insert(Commit {
                    id: CommitId {
                        user_id: self.user_id,
                        counter: self.commits.len(),
                    },
                    change,
                });
            }
            Message::Import => {
                self.commits.insert(Commit {
                    id: CommitId {
                        user_id: 1,
                        counter: 0,
                    },
                    change: 1,
                });
                self.commits.insert(Commit {
                    id: CommitId {
                        user_id: 1,
                        counter: 1,
                    },
                    change: 1,
                });
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // println!("Commits: {:#?}", self.commits);
        let number = self
            .commits
            .iter()
            .fold(0, |acc, commit| acc + commit.change);
        column![
            button("Increment").on_press(Message::Change(1)),
            text(number).size(50),
            button("Decrement").on_press(Message::Change(-1)),
            button("Import").on_press(Message::Import)
        ]
        .padding(20)
        .align_x(Center)
    }
}
