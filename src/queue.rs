use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct QueueShared<T> {
    slots: [Mutex<Option<T>>; 2],
    write_slot: AtomicBool,
}

#[derive(Debug)]
pub struct QueueSender<T> {
    shared: Arc<QueueShared<T>>,
    sender: tokio::sync::mpsc::Sender<()>,
}

impl<T> QueueSender<T> {
    pub fn send(&mut self, data: T) {
        let write_slot = self.shared.write_slot.load(Ordering::Relaxed);
        let mut slot = if let Ok(slot) = self.shared.slots[usize::from(write_slot)].try_lock() {
            slot
        } else {
            // The only way the write slot was locked is if the reader just finished reading the other slot
            // So the other slot must be not locked
            self.shared.slots[usize::from(!write_slot)]
                .try_lock()
                .unwrap()
        };
        *slot = Some(data);
        drop(slot);
        let _ = self.sender.try_send(());
    }
}

#[derive(Debug)]
pub struct QueueReceiver<T> {
    shared: Arc<QueueShared<T>>,
    receiver: tokio::sync::mpsc::Receiver<()>,
}

impl<T> QueueReceiver<T> {
    pub async fn recv(&mut self) -> T {
        self.receiver.recv().await.unwrap();
        // Tell the writer to write to the other slot
        let slot_index = self.shared.write_slot.fetch_not(Ordering::Relaxed);
        // Wait for the writer to finish writing to this slot and release the lock, if it was in the middle of writing
        self.shared.slots[usize::from(slot_index)]
            .lock()
            .await
            .take()
            .unwrap()
    }
}

/// Creates a reader and writer, where intermediary writes don't matter, just the latest write matters, since writes overwrite each other.
pub fn queue<T>() -> (QueueSender<T>, QueueReceiver<T>) {
    let shared = Arc::new(QueueShared {
        slots: [Mutex::new(None), Mutex::new(None)],
        write_slot: Default::default(),
    });
    let (sender, receiver) = tokio::sync::mpsc::channel(1);
    (
        QueueSender {
            shared: shared.clone(),
            sender,
        },
        QueueReceiver {
            shared: shared,
            receiver,
        },
    )
}
