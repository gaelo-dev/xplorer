/// Wrap the crate btleplug for more comfortable use
use super::{Result, BlueError};

use btleplug::api::{Central as _, Characteristic, Manager as _, Peripheral as _, PeripheralProperties, ScanFilter, ValueNotification, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use std::{time::Duration, fmt::Display};
use iced::futures::StreamExt;
use tokio::time;
use uuid::Uuid;

pub type Peripherals = Vec<(PeripheralId, PeripheralProperties)>;

const SERVICE_UUID: Uuid = uuid::uuid!("0000FFE0-0000-1000-8000-00805F9B34FB");
const CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000FFE1-0000-1000-8000-00805F9B34FB");

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
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;
        // let adapter = get_adapter(manager).await?;
        
        Ok(Self { 
            adapter: get_adapter(manager).await?, 
            filter: SERVICE_UUID
        })
    }

    /// Starts a scan for BLE devices. This scan filters out most devices.
    pub async fn scan(&self) -> Result<Peripherals> {
        let filter = ScanFilter { services: vec![self.filter] }; 
        self.adapter.start_scan(filter).await?;
        time::sleep(Duration::from_secs(15)).await;
        
        self.peripherals().await
    }

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
        if !xplorer.is_connected().await {
            xplorer.connect().await?;
        }
        
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
    /// Create an instance of the Bluetooth communication with the explorer
    /// 
    /// Use [`Central::connect`]
    fn new(peripheral: Peripheral) -> Self {
        let id = peripheral.id();

        Self { 
            peripheral,
            id 
        }
    }

    pub async fn is_connected(&self) -> bool {
        match self.peripheral.is_connected().await {
            Ok(is_connected) => is_connected,
            _ => false,
        }
    }

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
        // self.subscribe().await?;

        Ok(())
    }

    pub fn disconnect(&self) -> Result<()> {
        todo!()
    }

    /// Send a message/command to the explorer
    /// 
    /// The HM-10 only has one writable characteristic: [`CHARACTERISTIC_UUID`]
    pub async fn send<T: Display>(&self, msg: T) -> Result<()> {    
        let characteristic = self.charactacteristic()?;
        Ok(self.peripheral
            .write(&characteristic, msg.to_string().as_bytes(), WriteType::WithoutResponse)
            .await?)
    }

    pub async fn recv(&self) -> Result<Vec<u8>> {
        let characteristic = self.charactacteristic()?;
        Ok(self.peripheral
            .read(&characteristic)
            .await?)
    }

    pub async fn recv_notifications(&self, lot: usize) -> Result<Vec<ValueNotification>> {
        let stream = self.peripheral.notifications().await?.take(lot);
        Ok(stream.collect().await)
    }
}
