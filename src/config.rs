use crate::error::SanupResult;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

pub trait Config: Serialize + for<'a> Deserialize<'a> + Default {
    fn init<P: AsRef<Path>>(path: P) -> SanupResult<Self> {
        let path = path.as_ref();

        if path.exists() {
            Self::load(path)
        } else {
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> SanupResult<()> {
        let path = path.as_ref();
        let toml_string = toml::to_string(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> SanupResult<Self> {
        let path = path.as_ref();

        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config = toml::from_str(content.as_str())?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}
