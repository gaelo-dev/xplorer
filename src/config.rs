use crate::bluetooth;
use serde::{Deserialize, Serialize};

/// For some reason when deserializing the [`bluetooth::BDAddr`] type it gives an error, 
/// for now this type works to deserialize the addr as a string
#[derive(Default, Debug, Serialize, Deserialize)]
struct _Config {
    addr: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub addr: Option<bluetooth::BDAddr>,
}

impl Config {
    pub fn load() -> Result<Self, confy::ConfyError> {
        let cfg: _Config = confy::load(env!("CARGO_PKG_NAME"), None)?;
        Ok(cfg.into())
    }

    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store(env!("CARGO_PKG_NAME"), None, self)
    }
}

impl From<_Config> for Config {
    fn from(value: _Config) -> Self {
        let addr = value.addr.map(|ip| ip.parse().unwrap());
        Self { addr }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        self.save().unwrap();
    }
}
