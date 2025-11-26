use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct WatchedDisk {
    id: Uuid,
    label: Option<String>,
    mount_path: PathBuf,
    backup_profile: Option<Uuid>,
}
