use std::{collections::HashSet, path::Path, sync::Arc, time::Duration};

use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use tokio::{
    fs::{OpenOptions, write},
    io::{self, AsyncReadExt},
    time::sleep,
};

use crate::Commit;

#[derive(Debug, Clone)]
pub struct WriteRequest {
    pub id: usize,
    pub contents: HashSet<Commit<i64>>,
}

#[derive(Debug, Clone)]
pub struct WriteResponse {
    pub id: usize,
    pub result: Arc<Result<(), io::Error>>,
}

#[derive(Debug, Clone)]
pub struct ReadResponse {
    pub id: usize,
    pub result: Arc<Result<HashSet<Commit<i64>>, io::Error>>,
}

#[derive(Debug, Clone)]
pub enum FileStorageMessage {
    Read(ReadResponse),
    Write(WriteResponse),
}

pub fn subscription(
    path: impl AsRef<Path> + Send + Sync,
    read_receiver: flume::Receiver<usize>,
    write_receiver: flume::Receiver<WriteRequest>,
) -> impl Stream<Item = FileStorageMessage> + Send {
    stream::channel(1000, async move |mut output| {
        let try_load = async || {
            sleep(Duration::from_secs(2)).await;
            let mut string = String::new();
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&path)
                .await?
                .read_to_string(&mut string)
                .await?;
            let commits = if string.is_empty() {
                Default::default()
            } else {
                ron::from_str(&string).unwrap()
            };
            Ok(commits)
        };
        let mut loaded = false;
        loop {
            enum Request {
                Read(usize),
                Write(WriteRequest),
            }
            let request = tokio::select! {
                read_request = read_receiver.recv_async() => Request::Read(read_request.unwrap()),
                write_request = write_receiver.recv_async() => Request::Write(write_request.unwrap()),
            };
            match request {
                Request::Read(id) => {
                    if !loaded {
                        output
                            .send(FileStorageMessage::Read(ReadResponse {
                                id,
                                result: Arc::new(try_load().await),
                            }))
                            .await
                            .unwrap();
                    }
                }
                Request::Write(WriteRequest { id, mut contents }) => {
                    if !loaded {
                        contents.extend(try_load().await.unwrap());
                        loaded = true;
                    }
                    sleep(Duration::from_secs(3)).await;
                    output
                        .send(FileStorageMessage::Write(WriteResponse {
                            id,
                            result: Arc::new(
                                write(
                                    &path,
                                    ron::ser::to_string_pretty(&contents, Default::default())
                                        .unwrap(),
                                )
                                .await,
                            ),
                        }))
                        .await
                        .unwrap();
                }
            }
        }
    })
}
