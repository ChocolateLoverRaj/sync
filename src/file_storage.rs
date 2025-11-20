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

use crate::queue::QueueReceiver;

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
    mut read_receiver: QueueReceiver<usize>,
    mut write_receiver: QueueReceiver<WriteRequest>,
) -> impl Stream<Item = FileStorageMessage> + Send {
    stream::channel(1000, async move |mut output| {
        loop {
            enum Request {
                Read(usize),
                Write(WriteRequest),
            }
            let request = tokio::select! {
                read_request = read_receiver.recv() => Request::Read(read_request),
                write_request = write_receiver.recv() => Request::Write(write_request),
            };
            match request {
                Request::Read(id) => {
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
                Request::Write(WriteRequest { id, contents }) => {
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
    })
}
