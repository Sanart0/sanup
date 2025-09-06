use std::fmt::Display;
use log::error;

pub type SanupResult<T> = Result<T, SanupError>;

#[derive(Debug)]
pub enum SanupError {
    None,
    IO(std::io::Error),
    Poison(String),
    SerializeToml(toml::ser::Error),
    DeserializeToml(toml::de::Error),
    MongoDB(mongodb::error::Error),
    SetLogger(log::SetLoggerError),
    Other(String),
}

impl From<()> for SanupError {
    fn from(_value: ()) -> Self {
        error!("{:?}", _value);
        Self::None
    }
}

impl From<std::io::Error> for SanupError {
    fn from(value: std::io::Error) -> Self {
        error!("{}", value);
        Self::IO(value)
    }
}

impl<T> From<std::sync::PoisonError<T>> for SanupError {
    fn from(value: std::sync::PoisonError<T>) -> Self {
        error!("{}", value);
        Self::Poison(value.to_string())
    }
}

impl From<toml::ser::Error> for SanupError {
    fn from(value: toml::ser::Error) -> Self {
        error!("{}", value);
        Self::SerializeToml(value)
    }
}

impl From<toml::de::Error> for SanupError {
    fn from(value: toml::de::Error) -> Self {
        error!("{}", value);
        Self::DeserializeToml(value)
    }
}

impl From<mongodb::error::Error> for SanupError {
    fn from(value: mongodb::error::Error) -> Self {
        error!("{}", value);
        Self::MongoDB(value)
    }
}

impl From<log::SetLoggerError> for SanupError {
    fn from(value: log::SetLoggerError) -> Self {
        error!("{}", value);
        Self::SetLogger(value)
    }
}

impl std::error::Error for SanupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SanupError::None => None,
            SanupError::IO(err) => Some(err),
            SanupError::Poison(_err) => None,
            SanupError::SerializeToml(err) => Some(err),
            SanupError::DeserializeToml(err) => Some(err),
            SanupError::MongoDB(err) => Some(err),
            SanupError::SetLogger(err) => Some(err),
            SanupError::Other(_) => None,
        }
    }
}

impl Display for SanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SanupError::None => String::new(),
                SanupError::IO(err) => err.to_string(),
                SanupError::Poison(err) => err.to_string(),
                SanupError::SerializeToml(err) => err.to_string(),
                SanupError::DeserializeToml(err) => err.to_string(),
                SanupError::MongoDB(err) => err.to_string(),
                SanupError::SetLogger(err) => err.to_string(),
                SanupError::Other(err) => err.to_string(),
            }
        )
    }
}
