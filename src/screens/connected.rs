// to do!
use std::sync::Arc;

use iced::{
    futures::{channel::mpsc, SinkExt, lock::Mutex}, 
    widget::{column, row, vertical_rule, slider, text},
    keyboard, 
    Element, Subscription, Task 
};

use crate::bluetooth::{arm, car, Command};

pub enum Action {
    Run(Task<Message>),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Command(Command),
    CommandReceived(Command), 
    SliderChanged((usize, u8)),
    SliderCommand(usize),
    Ok,
}

#[derive(Debug, Clone)]
pub struct Connected {
    command: Command,
    sender: Arc<Mutex<mpsc::Sender<Command>>>,
    sliders: Vec<Slider>,
    sensors: Vec<Sensor>,
}

impl Connected {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self { 
            command: Command::default(),
            sender: Arc::new(Mutex::new(tx)),
            sliders: vec![
                // Car
                Slider { name: "Speed", cmd: car::speed(100), range: 0..=255 },

                // Arm
                Slider { name: "Base", cmd: arm::base(90), range: 0..=180 }, Slider { name: "Elbow", cmd: arm::elbow(90), range: 0..=180 },
                Slider { name: "Rest", cmd: arm::rest(90), range: 0..=180 }, Slider { name: "Shoulder", cmd: arm::shoulder(90), range: 0..=180 },
                Slider { name: "Doll", cmd: arm::doll(90), range: 0..=180 }, Slider { name: "Grip", cmd: arm::grip(90), range: 0..=180 },
            ],
            sensors: vec![
                Sensor { id: 1 << 0,name: "Distancia", value: 0 }, Sensor { id: 1 << 1, name: "Temperatura", value: 0 }, 
                Sensor { id: 1 << 2, name: "Humedad", value: 0 }, Sensor { id: 1 << 3, name: "Gas", value: 0 }
            ],
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Command(cmd) => {
                if self.command != cmd {
                    self.command = cmd;
                    let sender = Arc::clone(&self.sender); 

                    Action::Run(
                        Task::perform(
                            async move {
                                let mut sender = sender.lock().await;
                                let _ = sender.send(cmd).await;
                            },
                            |_| Message::Ok,
                        )
                    )
                } else {
                    Action::None
                }
            },
            Message::CommandReceived(cmd) => {
                if let Some(value) = cmd.value {
                    let sensor = self.sensors.iter_mut().find(|sensor| cmd.action == sensor.id).unwrap();
                    sensor.value = value;

                }
                
                Action::None
            },
            Message::SliderChanged((num, value)) => {
                let cmd = &mut self.sliders[num].cmd;
                cmd.value = Some(value);

                Action::None
            },
            Message::SliderCommand(num) => Action::Run(Task::done(Message::Command((&self.sliders[num].cmd).clone()))),
            Message::Ok => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sliders = self.sliders.iter().enumerate().map(|(n, slr)| {
            column![
                text(slr.name),
                slider(slr.range.clone(), slr.cmd.value.unwrap(), move |v| Message::SliderChanged((n, v))).on_release(Message::SliderCommand(n))
            ].into()
        });

        let sensors = self.sensors.iter().map(|sensor| text(format!("{}: {}", sensor.name, sensor.value)).into());

        row![
            column(sliders),
            vertical_rule(5),
            column(sensors)
        ].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::on_key_press(|key, modifiers| {
                match key.as_ref() {
                    // Car -> WASD
                    keyboard::Key::Character("w") => Some(car::forward()),
                    keyboard::Key::Character("s") => Some(car::backward()),
                    keyboard::Key::Character("a") => Some(car::leftward()),
                    keyboard::Key::Character("d") => Some(car::leftward()),
                    
                    // Arm 
                    // ...
                    _ => None,
                }
                .map(|cmd| if modifiers.shift() { cmd + car::speed(255) } else { cmd })
                .map(Message::Command)
            }),

            keyboard::on_key_release(|key, _modifiers| {
                match key.as_ref() {
                    keyboard::Key::Character("w" | "s" | "a" | "d") => Some(car::speed(0)),
                    _ => None,
                }
                .map(Message::Command)
            }),
        ])
    }
}

#[derive(Debug, Clone)]
struct Slider {
    name: &'static str,
    cmd: Command,
    range: std::ops::RangeInclusive<u8>,
}

#[derive(Debug, Clone)]
struct Sensor {
    id: u8,
    name: &'static str,
    value: u8,
}
