use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CompressionKind {
    None,
    Zip,
}
