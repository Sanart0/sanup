use log::Level;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    app::{
        compression_kind::CompressionKind, compression_level::CompressionLevel,
        hash_algorithm::HashAlgorithm, theme::Theme, watched_disk::WatchedDisk,
    },
    config::Config,
};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    default_backup_dir: PathBuf,
    compression_enable: bool,
    compression_kind: CompressionKind,
    compression_level: CompressionLevel,
    check_free_space_before_backup: bool,
    min_free_space_gb: u64,
    verify_after_backup: bool,
    hash_algorithm: HashAlgorithm,
    theme: Theme,
    show_hidden_files: bool,
    log_level: Level,
    log_to_file: bool,
    log_file_dir: Option<PathBuf>,
    watched_disk: Vec<WatchedDisk>,
}

impl Settings {
    pub fn fields(&self) {}
}

impl Config for Settings {}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            default_backup_dir: dirs::home_dir()
                .map(|h| h.join("Backups"))
                .unwrap_or_else(|| PathBuf::from("./Backups")),
            compression_enable: false,
            compression_kind: CompressionKind::None,
            compression_level: CompressionLevel::Default,
            check_free_space_before_backup: true,
            min_free_space_gb: 10,
            verify_after_backup: true,
            hash_algorithm: HashAlgorithm::Sha256,
            theme: Theme::Dark,
            show_hidden_files: true,
            log_level: Level::Info,
            log_to_file: false,
            log_file_dir: None,
            watched_disk: Vec::new(),
        }
    }
}
