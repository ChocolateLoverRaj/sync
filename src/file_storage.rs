use std::{collections::HashSet, path::Path, time::Duration};

use tokio::{
    fs::OpenOptions,
    io::{self, AsyncReadExt},
    time::sleep,
};

use crate::commit::Commit;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ReadError {
    Open(io::Error),
    Read(io::Error),
    Deserialize(ron::de::SpannedError),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WriteError {
    Write(io::Error),
    Serialize(ron::Error),
}

pub async fn read(path: impl AsRef<Path>) -> Result<HashSet<Commit<i64>>, ReadError> {
    sleep(Duration::from_secs(2)).await;
    let mut string = String::new();
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .await
        .map_err(ReadError::Open)?
        .read_to_string(&mut string)
        .await
        .map_err(ReadError::Read)?;
    let commits = if string.is_empty() {
        Default::default()
    } else {
        ron::from_str(&string).map_err(ReadError::Deserialize)?
    };
    Ok(commits)
}

pub async fn write(
    path: impl AsRef<Path>,
    contents: &HashSet<Commit<i64>>,
) -> Result<(), WriteError> {
    sleep(Duration::from_secs(2)).await;
    tokio::fs::write(
        &path,
        ron::ser::to_string_pretty(&contents, Default::default()).map_err(WriteError::Serialize)?,
    )
    .await
    .map_err(WriteError::Write)?;
    Ok(())
}
