pub enum BackupStatus {
    Pending,
    Running { progress: f32, current_file: String },
    Paused,
    Completed,
    Failed { reason: String },
    Cancelled,
}
