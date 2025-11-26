use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::backup::kind::BackupKind;

pub struct BackupMetadata {
    id: Uuid,
    name: String,
    kind: BackupKind,
    source_path: PathBuf,
    target_path: PathBuf,
    file_count: u64,
    total_size_bytes: u64,
    archive_checksum: Option<String>,
    created_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
    duration: Option<Duration>,
    failed_files: Vec<String>,
    note: Option<String>,
}
