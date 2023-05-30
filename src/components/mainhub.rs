use std::{collections::HashMap, error::Error, sync::Mutex};
use narwhal_tooth::{scan_bluetooth, bluetooth::{BluetoothConnection, connect_peripheral}};
use crate::api::device::{Device, Hub};

use super::{led::new_led_device, dist_checker::new_dist_checker};

lazy_static::lazy_static! {
    pub static ref EVENT_POOL: Mutex<Vec<(String, Vec<u8>)>> = Mutex::new(vec![]);
}

pub fn pull() -> Vec<(String, Vec<u8>)> {
    let mut buffer = EVENT_POOL.lock().unwrap();
    let cloned = buffer.clone();
    *buffer = vec![];
    return cloned;
}

#[derive(Clone)]
pub struct MainHub {
    devices: Vec<Device>,
    connection: Option<BluetoothConnection>,
}

impl MainHub {
    pub async fn reconnect() -> Option<BluetoothConnection> {
        async {
            let scan_results = scan_bluetooth(3).await;
            let hub_device = scan_results.get_by_name("HMSoft").await.ok_or("Couldn't find HMSoft device")?;
            let opt_conn = connect_peripheral(&hub_device).await;

            if let Ok(conn) = &opt_conn {
                futures::executor::block_on(async {
                    conn.subscribe(|v| {
                        EVENT_POOL.lock().unwrap().push(v);
                    }).await.unwrap();
                });
            }

            opt_conn
        }.await.ok()
    }

    pub async fn new() -> Self {
        let connection = Self::reconnect().await;

        MainHub {
            devices: vec![ new_led_device(), new_dist_checker() ],
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
                connection.write(data.as_bytes()).await?;
            }
        }

        Ok(())
    }

    async fn retreive(&mut self, target: Device) -> Result<String, Box<dyn Error>> {
        let connection = if !self.is_valid().await {
            Self::reconnect().await
        } else {
            self.connection.clone()
        }.ok_or("Invalid connection")?;

        self.connection = Some(connection.clone());

        if target.id == "718b88cf-5df5-418f-aa19-3815cfcdde05" {
            connection.write("request".as_bytes()).await?;
        }
        let vec = pull();
        if let Some(data) = vec.first() {
            return Ok(String::from_utf8(data.1.clone()).unwrap());
        }
        Ok("".to_string())
    }

    async fn finish(&self) -> Result<(), Box<dyn Error>> {
        if self.connection.is_none() {
            return Ok(());
        }
        self.connection.clone().unwrap().disconnect().await.unwrap();

        Ok(())
    }

    async fn is_valid(&self) -> bool {
        if let Some(value) = self.connection.clone() {
            return value.peripheral_connected().await;
        }
        return false;
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