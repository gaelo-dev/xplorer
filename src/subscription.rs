use crate::bluetooth::{self, ConnectionState, Command};
use crate::Config;

use iced::{
    futures::{
        channel::mpsc, sink::SinkExt, Stream, StreamExt
    },
    stream, 
};

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    CommandReceived(Command),
    Disconnected{
        peripherals: bluetooth::Peripherals
    },
    Err(String),
    Ready(mpsc::Sender<Input>),
    // ...
}

#[derive(Debug, Clone)]
pub enum Input {
    Connect(bluetooth::BDAddr),
    Command(bluetooth::Command),
    // ...
}

pub fn connection() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let mut state = ConnectionState::Loading;

        let (sender, mut receiver) = mpsc::channel(100);        
        let _= output.send(Event::Ready(sender)).await;
        
        loop {
            match &mut state {
                ConnectionState::Connected { xplorer, .. } => {
                    let mut notifications = xplorer.notifications().await.unwrap();

                    iced::futures::select! {
                        input = receiver.select_next_some() => {
                            if let Input::Command(cmd) = input {
                                if let Err(err) = xplorer.send(cmd).await {
                                    let _= output.send(Event::Err(err.to_string())).await;
                                }
                            }
                        }

                        notification = notifications.select_next_some() => {
                            let cmd = Command::from(notification.value);
                            let _= output.send(Event::CommandReceived(cmd)).await;
                        }
                        
                    }
                },
                ConnectionState::Disconnected { .. } => {
                    if let Input::Connect(addr) = receiver.select_next_some().await {       
                        println!("trying connect to: {addr}");                 
                        state = match state.clone().reconnect(addr).await {
                            Ok(s) => {
                                let _= output.send(Event::Connected).await;
                                s
                            },
                            Err(err) => {
                                let _= output.send(Event::Err(err.to_string())).await;
                                state
                            }
                        }
                    }
                },
                ConnectionState::Loading => {
                    let cfg = Config::load().unwrap();

                    match bluetooth::start(cfg.addr).await {
                        Ok(s) => {
                            state = s;
                            match &state {
                                ConnectionState::Connected { .. } => {
                                    let _ = output.send(Event::Connected).await;
                                },
                                ConnectionState::Disconnected { peripherals, .. } => {
                                    let _ = output.send(Event::Disconnected { peripherals: peripherals.clone() }).await;
                                }
                                _ => ()
                            }
                        },
                        Err(err) => {
                            let _= output.send(Event::Err(err.to_string())).await;
                        }
                    }
                },
            }
        }
    })
}
