use std::{collections::HashSet, mem, ops::Deref, path::PathBuf, sync::Arc};

use iced::{
    Task,
    futures::{FutureExt, StreamExt},
};
use id_factory::untyped::{Id, IdFactory};
use tokio::{select, sync::Mutex};

use crate::{
    commit::{Commit, Crdt},
    file_storage::{ReadError, WriteError, read, write},
};

pub struct State<T: Crdt> {
    commits: HashSet<Commit<T::CommitData>>,
    paths: Arc<Mutex<Paths>>,
    load_status: LoadStatus,
    save_id_factory: IdFactory,
    save_status: SaveStatus,
}

#[derive(Debug, Clone)]
pub enum Message<T: Crdt> {
    Load,
    LoadResult(Result<Arc<HashSet<Commit<T::CommitData>>>, Arc<ReadError>>),
    Save,
    SaveResult(SaveResult),
    Commit(T::CommitData),
}

#[derive(Debug, Clone)]
struct SaveResult {
    id: Id,
    result: Result<(), Arc<WriteError>>,
}

#[derive(Debug)]
pub enum LoadStatus {
    NotLoaded(Option<Arc<ReadError>>),
    Loading,
    Loaded,
}

#[derive(Debug)]
pub enum SaveStatus {
    NotSaving(Option<SaveError>),
    WaitingForLoad,
    Saving(SavingData),
}

#[derive(Debug)]
pub struct SavingData {
    id: Id,
    cancel: tokio::sync::oneshot::Sender<()>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SaveError {
    LoadError(Arc<ReadError>),
    WriteError(Arc<WriteError>),
}

#[derive(Debug)]
pub struct Paths {
    pub file: PathBuf,
    pub temp_file: PathBuf,
}

impl<T: Crdt> State<T> {
    // new
    pub fn new(paths: Paths) -> Self {
        Self {
            commits: Default::default(),
            paths: Arc::new(paths.into()),
            load_status: LoadStatus::NotLoaded(None),
            save_id_factory: Default::default(),
            save_status: SaveStatus::NotSaving(None),
        }
    }

    fn update_load(&mut self) -> Task<Message<T>> {
        match &self.load_status {
            LoadStatus::NotLoaded(_) => {
                self.load_status = LoadStatus::Loading;
                let paths = self.paths.clone();
                Task::future(async move {
                    Message::LoadResult(
                        read::<T>(paths.lock().await.file.deref())
                            .await
                            .map(Arc::new)
                            .map_err(Arc::new),
                    )
                })
            }
            _ => Task::none(),
        }
    }

    fn update_save(&mut self) -> Task<Message<T>> {
        match self.load_status {
            LoadStatus::NotLoaded(_) => {
                self.save_status = SaveStatus::WaitingForLoad;
                self.update_load()
            }
            LoadStatus::Loading => {
                self.save_status = SaveStatus::WaitingForLoad;
                Task::none()
            }
            LoadStatus::Loaded => {
                let (tx, rx) = tokio::sync::oneshot::channel();
                let id = self.save_id_factory.next_id();
                if let SaveStatus::Saving(saving_data) = mem::replace(
                    &mut self.save_status,
                    SaveStatus::Saving(SavingData { id, cancel: tx }),
                ) {
                    let _ = saving_data.cancel.send(());
                };
                let commits = self.commits.clone();
                let paths = self.paths.clone();
                Task::stream(
                    async move {
                        let paths = select! {
                            _ = rx => None,
                            path = paths.lock() => Some(path)
                        }?;
                        Some(Message::SaveResult(SaveResult {
                            id,
                            result: write::<T>(
                                paths.file.deref(),
                                paths.temp_file.deref(),
                                &commits,
                            )
                            .await
                            .map_err(Arc::new),
                        }))
                    }
                    .into_stream()
                    .filter_map(async |x| x),
                )
            }
        }
    }

    // update
    pub fn update(&mut self, message: Message<T>) -> Task<Message<T>> {
        match message {
            Message::Load => self.update_load(),
            Message::LoadResult(result) => match result {
                Ok(commits) => {
                    self.commits.extend(commits.iter().cloned());
                    self.load_status = LoadStatus::Loaded;
                    if let SaveStatus::WaitingForLoad = self.save_status {
                        self.update_save()
                    } else {
                        Task::none()
                    }
                }
                Err(e) => {
                    self.load_status = LoadStatus::NotLoaded(Some(e.clone()));
                    if let SaveStatus::WaitingForLoad = self.save_status {
                        self.save_status = SaveStatus::NotSaving(Some(SaveError::LoadError(e)))
                    }
                    Task::none()
                }
            },
            Message::Save => self.update_save(),
            Message::SaveResult(SaveResult { id, result }) => {
                if let SaveStatus::Saving(saving_data) = &self.save_status
                    && id == saving_data.id
                {
                    self.save_status =
                        SaveStatus::NotSaving(result.err().map(SaveError::WriteError))
                } else {
                    // we don't care, regardless of if it failed or succeeded
                    // in the future we might care if we keep track of outdated but saved versions
                }
                Task::none()
            }
            Message::Commit(change) => {
                self.commits.insert(Commit::new(change));
                self.update_save()
            }
        }
    }

    // used in the `view` fn to show data
    pub fn value(&self) -> T::Value {
        T::compute(self.commits.iter().map(|commit| *commit.data()))
    }

    pub fn commits_len(&self) -> usize {
        self.commits.len()
    }

    pub fn load_status(&self) -> &LoadStatus {
        &self.load_status
    }

    pub fn save_status(&self) -> &SaveStatus {
        &self.save_status
    }

    // used in the `view` fn to do actions
    pub fn change(&self, change: T::CommitData) -> Message<T> {
        Message::Commit(change)
    }

    pub fn load(&self) -> Message<T> {
        Message::Load
    }

    pub fn save(&self) -> Message<T> {
        Message::Save
    }
}
