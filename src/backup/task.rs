use crate::backup::{metadata::BackupMetadata, status::BackupStatus, worker::BackupWorker};
use uuid::Uuid;

pub struct BackupTask {
    id: Uuid,
    metadata: BackupMetadata,
    status: BackupStatus,
    worker: Option<BackupWorker>,
}
