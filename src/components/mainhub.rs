use std::collections::HashMap;
use narwhal_tooth::{scan_bluetooth, bluetooth::{BluetoothConnection, connect_peripheral}};
use crate::api::device::{Device, Hub};

use super::led::new_led_device;

#[derive(Clone)]
pub struct MainHub {
    devices: Vec<Device>,
    connection: BluetoothConnection
}

impl MainHub {
    pub async fn new() -> Self {
        let scan_results = scan_bluetooth(3).await;
        let hub_device = scan_results.get_by_name("HMSoft").await.expect("Could not find HMSoft device");
        let connection = connect_peripheral(&hub_device).await.unwrap();

        MainHub {
            devices: vec![ new_led_device() ],
            connection
        }
    }
}

#[async_trait::async_trait]
impl Hub for MainHub {
    async fn apply(&self, target: Device, data: &HashMap<String, String>) {
        if target.id == "287a47cc-f0fa-4575-948a-ffec1e1c7c7b" {
            if let Some(data) = data.get("status") {
                self.connection.write(data.as_bytes()).await.unwrap();
            }
        }
    }

    async fn finish(&self) {
        self.connection.disconnect().await.unwrap();
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