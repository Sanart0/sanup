use crate::backup::status::BackupStatus;
use uuid::Uuid;

pub enum Message {
    Ping(),
    Pong(Uuid, BackupStatus),
}
