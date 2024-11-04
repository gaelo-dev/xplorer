mod result {
    pub(super) type Result<T> = std::result::Result<T, BlueError>;
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
mod actions;

use result::{Result, BlueError};
use wrap::{Central, Peripherals, Xplorer};
pub use actions::{Servo, Motor, Action};
pub use btleplug::api::BDAddr;

/// Represents the connection state of the Bluetooth communication
#[derive(Debug, Clone)]
pub enum ConnectionState {
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

/// Try to start the bluetooth connection. Return a [`ConnectionState`]
/// 
/// **Parameters**
/// 
/// peripheral_id -> If an IP is passed, the connection with peripheral will be attempted
pub async fn start(peripheral_ip: Option<BDAddr>) -> Result<ConnectionState> {
    let central = Central::new().await.unwrap();
    let peripherals = central.scan().await?;

    let state = ConnectionState::Disconnected { central, peripherals };

    match peripheral_ip {
        Some(ip) => Ok(state.reconnect(ip).await?),
        None => Ok(state)
    }
}
