use std::collections::HashSet;

use edits::{Edit, EditId, deserialize_edits, get_external_edits};
use iced::Center;
use iced::widget::{Column, button, column, text};
use tink_core::keyset::Handle;

mod edits;

pub fn main() -> iced::Result {
    tink_signature::init();
    tink_aead::init();
    iced::run("Sync Demo", Counter::update, Counter::view)
}

struct Counter {
    encryption_key: Handle,
    test_private_key: Handle,
    trusted_public_key: Handle,
    user_id: u64,
    edits: HashSet<Edit>,
}

impl Default for Counter {
    fn default() -> Self {
        let test_private_key =
            tink_core::keyset::Handle::new(&tink_signature::ed25519_key_template()).unwrap();
        Self {
            encryption_key: Handle::new(&tink_aead::aes256_gcm_key_template()).unwrap(),
            trusted_public_key: test_private_key.public().unwrap(),
            test_private_key,
            user_id: 0,
            edits: Default::default(),
        }
    }
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
                self.edits.insert(Edit {
                    id: EditId {
                        user_id: self.user_id,
                        counter: self.edits.len(),
                    },
                    change,
                });
            }
            Message::Import => {
                let external_edits =
                    get_external_edits(&self.encryption_key, &self.test_private_key);
                let edits = deserialize_edits(
                    &self.encryption_key,
                    &self.trusted_public_key,
                    &external_edits,
                );
                self.edits.extend(edits);
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // println!("Commits: {:#?}", self.commits);
        let number = self.edits.iter().fold(0, |acc, commit| acc + commit.change);
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
