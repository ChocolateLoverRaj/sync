use std::{path::Path, sync::Arc, time::Duration};

use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use tokio::{
    fs::{OpenOptions, write},
    io::{self, AsyncReadExt},
    time::sleep,
};

#[derive(Debug, Clone)]
pub struct WriteRequest {
    pub id: usize,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub struct WriteResponse {
    pub id: usize,
    pub result: Arc<Result<(), io::Error>>,
}

#[derive(Debug, Clone)]
pub struct ReadResponse {
    pub id: usize,
    pub result: Arc<Result<String, io::Error>>,
}

#[derive(Debug, Clone)]
pub enum FileStorageMessage {
    Read(ReadResponse),
    Write(WriteResponse),
}

pub fn subscription(
    path: impl AsRef<Path> + Send + Sync,
    mut read_receiver: tokio::sync::watch::Receiver<Option<usize>>,
    mut write_receiver: tokio::sync::watch::Receiver<Option<WriteRequest>>,
) -> impl Stream<Item = FileStorageMessage> + Send {
    stream::channel(1000, async move |mut output| {
        loop {
            enum Request {
                Read,
                Write,
            }
            let request = tokio::select! {
                _ = read_receiver.changed() => Request::Read,
                _ = write_receiver.changed() => Request::Write,
            };
            match request {
                Request::Read => {
                    let read_request = read_receiver.borrow_and_update().clone();
                    if let Some(id) = read_request {
                        sleep(Duration::from_secs(2)).await;
                        let mut string = String::new();
                        OpenOptions::new()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open(&path)
                            .await
                            .unwrap()
                            .read_to_string(&mut string)
                            .await
                            .unwrap();
                        output
                            .send(FileStorageMessage::Read(ReadResponse {
                                id,
                                result: Arc::new(Ok(string)),
                            }))
                            .await
                            .unwrap();
                    }
                }
                Request::Write => {
                    let contents = write_receiver.borrow_and_update().clone();
                    if let Some(WriteRequest { id, contents }) = contents {
                        sleep(Duration::from_secs(3)).await;
                        output
                            .send(FileStorageMessage::Write(WriteResponse {
                                id,
                                result: Arc::new(write(&path, contents).await),
                            }))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    })
}
