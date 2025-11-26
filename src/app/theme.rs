use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Kanagawa,
    Nord,
    Dracula,
    TokyoNight,
}
