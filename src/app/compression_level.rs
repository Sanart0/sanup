use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CompressionLevel {
    Fast,
    Default,
    Best,
}
