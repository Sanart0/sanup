use std::path::PathBuf;

use uuid::Uuid;

pub struct BackupDisk {
    id: Uuid,
    label: String,
    mount_path: PathBuf,
    total_capacity_bytes: u64,
    free_space_bytes: u64,
}
