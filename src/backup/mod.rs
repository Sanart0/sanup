pub mod destination;
pub mod filter;
pub mod kind;
pub mod source;
pub mod status;

use crate::{
    backup::{
        destination::BackupDestination, filter::BackupFilter, kind::BackupKind,
        source::BackupSource, status::BackupStatus,
    },
    error::SanupResult,
    sanup::message::Message,
};
use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};
use uuid::Uuid;

#[allow(dead_code)]
pub struct Backup {
    id: Uuid,
    sanup: Sender<Message>,
    title: String,
    kind: BackupKind,
    status: BackupStatus,
    filter: BackupFilter,
    source: BackupSource,
    destination: BackupDestination,
}

#[allow(clippy::new_ret_no_self)]
impl Backup {
    pub fn new(
        id: Uuid,
        sanup: Sender<Message>,
        title: String,
        kind: BackupKind,
        filter: BackupFilter,
        source: BackupSource,
        destination: BackupDestination,
    ) -> SanupResult<JoinHandle<()>> {
        let _backup = Self {
            id,
            sanup,
            title,
            kind,
            status: BackupStatus::Pause,
            filter,
            source,
            destination,
        };

        Ok(thread::Builder::new().name(id.to_string()).spawn(|| {})?)
    }
}
