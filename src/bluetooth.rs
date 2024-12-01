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

/// Try to start the bluetooth connection. Return a [`ConnectionState`]
/// 
/// **Parameters**
/// 
/// peripheral_id -> If an IP is passed, the connection with peripheral will be attempted
pub async fn start(peripheral_ip: Option<BDAddr>) -> Result<ConnectionState> {
    let central = Central::new().await?;
    let peripherals = central.scan().await?;

    let state = ConnectionState::Disconnected { central, peripherals };

    match peripheral_ip {
        Some(ip) => Ok(state.reconnect(ip).await?),
        None => Ok(state)
    }
}

/// Represents the connection state of the Bluetooth communication
#[derive(Debug, Clone)]
pub enum ConnectionState {
    /// Represents the loading state, it is only used to create a loading view
    Loading,
    /// Represents the disconnected state and contains a list of peripherals to which it can connect.
    Disconnected {
        central: Central,
        peripherals: Peripherals
    },
    /// Represents the connected state and contains the peripheral to which it is connected.
    Connected {
        central: Central,
        xplorer: Xplorer,
    }
}

impl ConnectionState {
    pub async fn reconnect(self, peripheral_ip: BDAddr) -> Result<Self> {
        match self {
            Self::Disconnected { central, peripherals } => {
                let (id, _properties) = peripherals
                    .into_iter()
                    .find(|(_id, properties)| properties.address == peripheral_ip)
                    .ok_or(BlueError::NotFoundPeripheral)?;
            
                let xplorer = central.connect(&id).await?;
                Ok(ConnectionState::Connected { central, xplorer })
            },
            _ => Ok(self)
        }
    }
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
