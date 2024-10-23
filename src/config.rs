use crate::bluetooth;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub ip: Option<bluetooth::BDAddr>,
}

impl Config {
    pub fn load() -> Result<Self, confy::ConfyError> {
        Ok(confy::load(env!("CARGO_PKG_NAME"), None)?)
    }

    pub fn save(self) -> Result<(), confy::ConfyError> {
        confy::store(env!("CARGO_PKG_NAME"), None, self)
    }
}
