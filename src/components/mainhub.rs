use std::{collections::HashMap, error::Error};
use narwhal_tooth::{scan_bluetooth, bluetooth::{BluetoothConnection, connect_peripheral}};
use crate::api::device::{Device, Hub};

use super::led::new_led_device;

#[derive(Clone)]
pub struct MainHub {
    devices: Vec<Device>,
    connection: Option<BluetoothConnection>
}

impl MainHub {
    pub async fn reconnect() -> Option<BluetoothConnection> {
        async {
            let scan_results = scan_bluetooth(3).await;
            let hub_device = scan_results.get_by_name("HMSoft").await.ok_or("Couldn't find HMSoft device")?;
            connect_peripheral(&hub_device).await
        }.await.ok()
    }

    pub async fn new() -> Self {
        let connection = Self::reconnect().await;

        MainHub {
            devices: vec![ new_led_device() ],
            connection
        }
    }
}

#[async_trait::async_trait]
impl Hub for MainHub {
    async fn apply(&mut self, target: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        let connection = if !self.is_valid().await {
            Self::reconnect().await
        } else {
            self.connection.clone()
        }.ok_or("Invalid connection")?;

        self.connection = Some(connection.clone());

        if target.id == "287a47cc-f0fa-4575-948a-ffec1e1c7c7b" {
            if let Some(data) = data.get("status") {
                connection.write(data.as_bytes()).await.unwrap();
            }
        }

        Ok(())
    }

    async fn finish(&self) -> Result<(), Box<dyn Error>> {
        if self.connection.is_none() {
            return Ok(());
        }
        self.connection.clone().unwrap().disconnect().await.unwrap();

        Ok(())
    }

    async fn is_valid(&self) -> bool {
        self.connection.is_some() && self.connection.clone().unwrap().valid().await // todo remove useless clones for BluetoothConnection
    }

    fn get_name(&self) -> &str {
        "MainHub"
    }

    fn get_desc(&self) -> &str {
        "방 불을 켜거나 끕니다"
    }

    fn get_devices(&self) -> Vec<Device> {
        self.devices.clone()
    }
}