mod result {
    pub type Result<T> = std::result::Result<T, BlueError>;
    use thiserror::Error;

    #[derive(Error, Debug, Clone)]
    pub enum BlueError {
        #[error("Unable to find adapters.")]
        NotFoundAdapters,

        #[error("BLE peripheral devices were not found")]
        NotFoundPeripheral,

        #[error("I'm a teapot: the device refuses to make coffee because it is a teapot")]
        Error418,
        
        #[error("{0}")]
        Other(String),
    }

    impl From<btleplug::Error> for BlueError {
        fn from(value: btleplug::Error) -> Self {
            match value {
                btleplug::Error::Other(err) => Self::Other(format!("{err:?}")),
                _ => Self::Other(value.to_string()),
            }
        }
    }
}

mod wrap;
mod command;

pub use result::{Result, BlueError};
pub use wrap::{Central, Peripherals, Xplorer, Notifications};
pub use command::{Command, car, arm, sensors};
pub use btleplug::api::BDAddr;

use btleplug::{platform::PeripheralId, api::PeripheralProperties};

/// Search in the list of provided peripherals for the peripheral that matches the bluetooth address
pub fn search(peripherals: &Peripherals, addr: BDAddr) -> Result<&(PeripheralId, PeripheralProperties)> {
    peripherals
        .into_iter()
        .find(|(_id, properties)| properties.address == addr)
        .ok_or(BlueError::NotFoundPeripheral)
}

/// A trait to convert a type to a [`Vec<u8>`] (bytes array), necessary for sending messages to devices
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

/// General implementation for a String
impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<_> = self.bytes().collect();
        vec.push(0);
        vec
    }
}
