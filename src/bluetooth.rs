mod result {
    pub type Result<T> = std::result::Result<T, BlueError>;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum BlueError {
        #[error("Unable to find adapters.")]
        NotFoundAdapters,

        #[error("BLE peripheral devices were not found")]
        NotFoundPeripheral,

        #[error("I'm a teapot: the device refuses to make coffee because it is a teapot")]
        Error418,
        
        #[error("Error: {0}")]
        Other(#[from] btleplug::Error),
    }
}

mod wrap;
mod command;

pub use result::{Result, BlueError};
pub use wrap::{Central, Peripherals, Xplorer};
pub use command::{Command, car, arm};
pub use btleplug::api::BDAddr;

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
