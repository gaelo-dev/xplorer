/// Wrap the crate btleplug
use super::{ToBytes, BlueError, Result};

use std::{time::Duration, pin::Pin, task};
use btleplug::api::{Central as _, Characteristic, Manager as _, Peripheral as _, PeripheralProperties, ScanFilter, ValueNotification, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use iced::futures::{Stream, StreamExt, stream::FusedStream};
use tokio::time;
use uuid::Uuid;

pub type Peripherals = Vec<(PeripheralId, PeripheralProperties)>;

const SERVICE_UUID: Uuid = uuid::uuid!("0000ffe0-0000-1000-8000-00805f9b34fb");
const CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");

async fn get_adapter(manager: Manager) -> Result<Adapter> {
    let central = manager
        .adapters()
        .await?
        .into_iter()
        .nth(0)
        .ok_or(BlueError::NotFoundAdapters)?;

    Ok(central)
}

/// A wrapper for the central device (aka: bluetooth adapter).
/// It is the entry point for communication with the different devices that the app needs.
/// 
/// pd: For now I only deployed one device (Xplorer)
#[derive(Debug, Clone)]
pub struct Central {
    adapter: Adapter,
    filter: Uuid,
}

impl Central {
    /// Constructs an instance [`Central`] 
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;

        Ok(Self { 
            adapter: get_adapter(manager).await?, 
            filter: SERVICE_UUID
        })
    }

    /// Starts a scan for BLE devices. This scan filters out most devices.
    /// 
    /// Returns the [Peripherals] that are discovered in 15 seconds
    pub async fn scan(&self) -> Result<Peripherals> {
        let filter = ScanFilter { services: vec![self.filter] }; 
        self.adapter.start_scan(filter).await?;
        time::sleep(Duration::from_secs(15)).await;
        
        self.peripherals().await
    }

    /// Returns the [Peripherals] that have been discovered so far 
    pub async fn peripherals(&self) -> Result<Peripherals> {
        let peripherals = self.adapter.peripherals().await?;
        if peripherals.is_empty() {
            return Err(BlueError::NotFoundPeripheral);
        }

        let mut peripherals_properties = Vec::new();
        for peripheral in peripherals {
            let properties = peripheral.properties().await?;
            if properties.is_none() {
                continue;
            }

            peripherals_properties.push((peripheral.id(), properties.unwrap()))
        }

        Ok(peripherals_properties)
    }

    /// Create an instance of the device and make the connection.
    pub async fn connect(&self, id: &PeripheralId) -> Result<Xplorer> {
        let xplorer = Xplorer::new(self.adapter.peripheral(id).await?);
        xplorer.connect().await?;

        Ok(xplorer)
    }
}

/// A wrapper for the peripheral device, handles all Bluetooth communication with the explorer
/// 
/// Important: the explorer uses an HM-10 BLE 4
#[derive(Debug, Clone)]
pub struct Xplorer {
    pub id: PeripheralId,
    peripheral: Peripheral,
}

impl Xplorer {
    /// Constructs an instance of the Bluetooth communication with the explorer
    /// 
    /// Use [`Central::connect`]
    fn new(peripheral: Peripheral) -> Self {
        let id = peripheral.id();

        Self { 
            id, 
            peripheral,
        }
    }

    /// Returns true if it is connected 
    pub async fn is_connected(&self) -> bool {
        match self.peripheral.is_connected().await {
            Ok(is_connected) => is_connected,
            _ => false,
        }
    }

    /// Returns the properties associated with the [Xplorer] device
    pub async fn properties(&self) -> PeripheralProperties {
        self.peripheral.properties().await.unwrap().unwrap()
    }

    fn charactacteristic(&self) -> Result<Characteristic> {
        let characteristic = self.peripheral
            .characteristics()
            .into_iter()
            .find(|charac| charac.uuid == CHARACTERISTIC_UUID)
            .ok_or(BlueError::Error418)?;
        
        assert_eq!(characteristic.service_uuid, SERVICE_UUID);
        Ok(characteristic)
    }

    pub async fn subscribe(&self) -> Result<()> {
        let characteristic = self.charactacteristic()?;
        Ok(self.peripheral
            .subscribe(&characteristic)
            .await?)
    }

    /// Establish the BLE connection
    pub async fn connect(&self) -> Result<()> {
        self.peripheral.connect().await?;
        self.peripheral.discover_services().await?;
        time::sleep(Duration::from_secs(15)).await;

        self.subscribe().await?;

        Ok(())
    }

    pub fn disconnect(&self) -> Result<()> {
        todo!()
    }

    /// Send a message/command to the explorer
    /// 
    /// The HM-10 only has one writable characteristic: [`CHARACTERISTIC_UUID`]
    pub async fn send<T: ToBytes>(&self, msg: T) -> Result<()> { 
        let characteristic = self.charactacteristic()?;
        Ok(self.peripheral
            .write(&characteristic, &msg.to_bytes(), WriteType::WithoutResponse)
            .await?)
    }

    pub async fn recv(&self) -> Result<Vec<u8>> {
        let characteristic = self.charactacteristic()?;
        Ok(self.peripheral
            .read(&characteristic)
            .await?)
    }

    /// Returns a stream of [Notifications]
    pub async fn notifications(&self) -> Result<Notifications> {
        Ok(Notifications { 
            stream: self.peripheral.notifications().await?, 
            terminated: false 
        })
    }
}

/// A wrapper for a stream of notifications that implements [`FusedStream`]
pub struct Notifications {
    stream: Pin<Box<dyn Stream<Item = ValueNotification> + Send>>,
    terminated: bool,
}

impl Stream for Notifications {
    type Item = ValueNotification;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.terminated {
            task::Poll::Ready(None)
        } else {
            match this.stream.poll_next_unpin(cx) {
                task::Poll::Ready(None) => {
                    this.terminated = true;
                    task::Poll::Ready(None)
                }
                other => other,
            }
        }
    }
}

impl FusedStream for Notifications {
    fn is_terminated(&self) -> bool {
        self.terminated
    }
}

impl std::fmt::Debug for Notifications {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Notifications")
            .field("terminated", &self.terminated)
            .finish()
    }
}
