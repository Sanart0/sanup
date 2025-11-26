use crate::backup::{disk::BackupDisk, message::Message};
use std::{sync::mpsc::Sender, thread::JoinHandle};

pub struct BackupWorker {
    handler: JoinHandle<()>,
    tx: Sender<Message>,
    disk: BackupDisk,
}
