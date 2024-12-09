use crate::bluetooth::{self, Command, BDAddr};

use iced::{
    futures::{
        channel::mpsc, sink::SinkExt, Stream, StreamExt,
    },
    stream, 
};

pub fn connection() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let (sender, receiver) = mpsc::channel(100);
        
        let central = bluetooth::Central::new().await.unwrap();
        let peripherals = central.scan().await.unwrap();

        let _ = output.send(Event::Disconnected { peripherals: peripherals.clone(), sender }).await;
        let mut state = ConnectionState::Disconnected { peripherals, receiver };

        loop {
            match &mut state {
                ConnectionState::Connected { xplorer, receiver } => {
                    let mut notifications = xplorer.notifications().await.unwrap();

                    iced::futures::select! {
                        cmd = receiver.select_next_some() => {
                            if let Err(err) = xplorer.send(cmd).await {
                                let _= output.send(Event::Err(err.to_string())).await;
                            }
                        }

                        notification = notifications.select_next_some() => {
                            let cmd = Command::from(notification.value);
                            let _= output.send(Event::CommandReceived(cmd)).await;
                        }
                        
                    }
                },
                ConnectionState::Disconnected { peripherals, receiver, .. } => {
                    let addr = receiver.select_next_some().await;               

                    let (id, _properties) = peripherals
                        .into_iter()
                        .find(|(_id, properties)| properties.address == addr)
                        .ok_or(bluetooth::BlueError::NotFoundPeripheral)
                        .unwrap();
                    
                    let xplorer = central.connect(&id).await.unwrap();
                    let (sender, receiver) = mpsc::channel(100);
                    
                    let _ = output.send(Event::Connected { addr, sender }).await;
                    state = ConnectionState::Connected { xplorer, receiver };
                },
            }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected {
        addr: BDAddr,
        sender: mpsc::Sender<Command>,
    },
    CommandReceived(Command),
    Disconnected {
        peripherals: bluetooth::Peripherals,
        sender: mpsc::Sender<bluetooth::BDAddr>,
    },
    Err(String),
    // ...
}

/// Represents the connection state of the Bluetooth communication
#[derive(Debug)]
enum ConnectionState {
    /// Represents the connected state and contains the peripheral to which it is connected.
    Connected {
        // central: Central,
        xplorer: bluetooth::Xplorer,
        receiver: mpsc::Receiver<Command>,
    },

    /// Represents the disconnected state and contains a list of peripherals to which it can connect.
    Disconnected {
        // central: Central,
        peripherals: bluetooth::Peripherals,
        receiver: mpsc::Receiver<BDAddr>
    },
}
