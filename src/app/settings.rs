use crate::{
    app::{
        compression_kind::CompressionKind, compression_level::CompressionLevel,
        hash_algorithm::HashAlgorithm, log_level::LogLevel, theme::Theme,
        watched_disk::WatchedDisk,
    },
    config::Config,
    ui::input::{
        boolfield::BoolField,
        enumfield::EnumField,
        field::{Field, Fields},
        inputfield::InputField,
        integerfield::IntegerField,
        stringfield::StringField,
    },
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
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
    log_level: LogLevel,
    log_to_file: bool,
    log_file_dir: Option<PathBuf>,
    watched_disk: Vec<WatchedDisk>,
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
            log_level: LogLevel::Info,
            log_to_file: false,
            log_file_dir: None,
            watched_disk: Vec::new(),
        }
    }
}

impl Into<Fields> for Settings {
    fn into(self) -> Fields {
        let settings = Settings::default();

        Fields::new(vec![
            Field::String(InputField::new_with_value(
                "default_backup_dir",
                StringField::from(settings.default_backup_dir),
            )),
            Field::Bool(InputField::new_with_value(
                "compression_enable",
                BoolField::from(settings.compression_enable),
            )),
            Field::Enum(InputField::new_with_value(
                "compression_kind",
                EnumField::from(self.compression_kind),
            )),
            Field::Enum(InputField::new_with_value(
                "compression_level",
                EnumField::from(self.compression_level),
            )),
            Field::Bool(InputField::new_with_value(
                "check_free_space_before_backup",
                BoolField::from(self.check_free_space_before_backup),
            )),
            Field::Integer(InputField::new_with_value(
                "min_free_space_gb",
                IntegerField::from(self.min_free_space_gb as i64),
            )),
            Field::Bool(InputField::new_with_value(
                "verify_after_backup",
                BoolField::from(self.verify_after_backup),
            )),
            Field::Enum(InputField::new_with_value(
                "hash_algorithm",
                EnumField::from(self.hash_algorithm),
            )),
            Field::Enum(InputField::new_with_value(
                "theme",
                EnumField::from(self.theme),
            )),
            Field::Bool(InputField::new_with_value(
                "show_hidden_files",
                BoolField::from(self.show_hidden_files),
            )),
            Field::Enum(InputField::new_with_value(
                "log_level",
                EnumField::from(self.log_level),
            )),
            Field::Bool(InputField::new_with_value(
                "log_to_file",
                BoolField::from(self.log_to_file),
            )),
            Field::String(InputField::new_with_value(
                "log_file_dir",
                StringField::from(self.log_file_dir),
            )),
        ])
    }
}
