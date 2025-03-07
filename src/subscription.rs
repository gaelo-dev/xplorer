use crate::bluetooth::{self, Command, BDAddr};

use thiserror::Error;
use iced::{
    futures::{
        self, channel::mpsc, sink::SinkExt, 
        Stream, StreamExt,
    },
    stream, 
};

/// A builder for an asynchronous worker with bidirectional communication that manages Bluetooth connections.
///
/// This asynchronous worker facilitates the setup and handling of the Bluetooth connection process,
/// including discovering peripherals, establishing connections, and handling communication.
/// 
/// ## Use 
/// It's intended to be used as an [`iced::Subscription`]
/// 
/// ```no_run
/// Subscription::run(bluetooth_connection);
/// ```
/// 
/// ## See also
/// - [`Event`] for details on events emitted by the subscription.
pub fn bluetooth_connection() -> impl Stream<Item = EventResult> {
    stream::try_channel(100, |mut output| async move {
        let (sender, receiver) = mpsc::channel(100);

        let central = bluetooth::Central::new().await?;
        let peripherals = central.scan().await?;

        let _ = output.send(Event::Disconnected { peripherals: peripherals.clone(), sender }).await;
        let mut state = ConnectionState::Disconnected { peripherals, receiver };

        log::info!("The first connection state -> {state:#?}");

        loop {
            match &mut state {
                ConnectionState::Connected { xplorer, receiver, notifications } => {
                    futures::select! {
                        cmd = receiver.select_next_some() => {
                            xplorer.send(cmd).await?;
                            log::debug!("Command sent -> {cmd:?}");
                        }

                        notification = notifications.select_next_some() => {
                            let cmd = Command::from(notification.value);
                            let _= output.send(Event::CommandReceived(cmd)).await;

                            log::debug!("Command received -> {cmd:?}");
                        }
                    }
                },
                ConnectionState::Disconnected { peripherals, receiver } => {
                    let addr = receiver.select_next_some().await;               
                    log::info!("Trying connect to -> {addr}");

                    let (id, _) = bluetooth::search(peripherals, addr)?;
                    let xplorer = central.connect(id).await?;

                    let (sender, receiver) = mpsc::channel(100);
                    let notifications = xplorer.notifications().await?;

                    let _ = output.send(Event::Connected { addr, sender }).await;
                    state = ConnectionState::Connected { xplorer, receiver, notifications };

                    log::info!("The connection state changed to -> {state:#?}");
                },
            }
        }
    })
}

pub type EventResult = Result<Event, Error>;

/// The events generated by the subscription: [`bluetooth_connection`]
#[derive(Debug, Clone)]
pub enum Event {
    /// Emitted when a connection is successfully established with a peripheral.
    Connected {
        addr: BDAddr,
        sender: mpsc::Sender<Command>,
    },

    /// Emitted when a command is received.
    CommandReceived(Command),
    
    /// The initial event, indicating that there is no connection to any peripheral.
    Disconnected {
        peripherals: bluetooth::Peripherals,
        sender: mpsc::Sender<bluetooth::BDAddr>,
    },
}

/// An error that can occur in the subscription
#[derive(Error, Debug, Clone)]
#[error(transparent)]
pub struct Error(#[from] bluetooth::BlueError); 

/// Represents the connection state of the Bluetooth communication
#[derive(Debug)]
enum ConnectionState {
    /// Represents the connected state and contains the peripheral to which it is connected.
    Connected {
        xplorer: bluetooth::Xplorer,
        receiver: mpsc::Receiver<Command>,
        notifications: bluetooth::Notifications,
    },

    /// Represents the disconnected state and contains a list of peripherals to which it can connect.
    Disconnected {
        peripherals: bluetooth::Peripherals,
        receiver: mpsc::Receiver<BDAddr>
    },
}
