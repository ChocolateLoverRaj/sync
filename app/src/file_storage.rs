use std::{collections::HashSet, path::Path};

use tokio::{
    fs::{OpenOptions, rename},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

use crate::*;

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
    Open(io::Error),
    Write(io::Error),
    Sync(io::Error),
    Rename(io::Error),
    Serialize(ron::Error),
}

pub async fn read<T: Crdt>(
    path: impl AsRef<Path>,
) -> Result<HashSet<Commit<T::CommitData>>, ReadError> {
    // sleep(Duration::from_secs(2)).await;
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

pub async fn write<T: Crdt>(
    path: impl AsRef<Path>,
    temp_path: impl AsRef<Path>,
    contents: &HashSet<Commit<T::CommitData>>,
) -> Result<(), WriteError> {
    // sleep(Duration::from_secs(2)).await;
    // In order for the write to be atomic, we have to follow these steps
    // Write to a temporary file in the same file system (so we'll use the same folder)
    // fsync the temporary file
    // Rename the temporary file to the permanent file
    // Optionally, to ensure that the rename was saved on the disk, fsync the dir
    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_path)
        .await
        .map_err(WriteError::Open)?;
    temp_file
        .write_all(
            ron::ser::to_string_pretty(&contents, Default::default())
                .map_err(WriteError::Serialize)?
                .as_ref(),
        )
        .await
        .map_err(WriteError::Write)?;
    temp_file.sync_all().await.map_err(WriteError::Sync)?;
    drop(temp_file);
    rename(temp_path, path).await.map_err(WriteError::Rename)?;
    Ok(())
}
