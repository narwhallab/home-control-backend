use std::{collections::HashMap, error::Error, time::Duration};
use log::warn;
use narwhal_tooth::{scan::scan_bluetooth, bluetooth::BluetoothConnection, util::connect_device};
use crate::api::{device::{Device, Hub, DeviceType}, copts::ControllerType};

use super::{led::new_led_device, dist_checker::new_dist_checker};

#[derive(Clone)]
pub struct MainHub {
    devices: Vec<Device>,
    connection: Option<BluetoothConnection>,
}

impl MainHub {
    pub async fn reconnect() -> Option<BluetoothConnection> {
        {
            let scan_results = scan_bluetooth(Duration::from_secs(3)).await;
            let hub_device = scan_results.search_by_name("HMSoft".to_string()).await?;
            let connection_result = connect_device(hub_device.clone()).await;

            connection_result
        }.ok()
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
        if !self.is_valid().await {
            return Err("Device isn't connected".into());
        }

        if target.dev_type != DeviceType::Commandable {
            return Err("Only Commandable Devices can run this function".into());
        }

        // todo: add methods manually for each device

        for opt in target.ctrl_opts.iter() {
            if let Some(data) = data.get(&opt.name) {
                let _result = self.connection.clone().unwrap().send(data.as_bytes()).await?; // todo: maybe the result will be useful??
            }
        }

        Ok(())
    }

    async fn retreive(&mut self, target: Device) -> Result<HashMap<String, String>, Box<dyn Error>> {
        if !self.is_valid().await {
            return Err("Device isn't connected".into());
        }

        if target.dev_type != DeviceType::Readable {
            return Err("Only Readable Devices can run this function".into());
        }

        let mut map: HashMap<String, String> = HashMap::new();

        for opt in target.ctrl_opts.iter() {
            if opt.opt_type == ControllerType::Read {
                let result = self.connection.clone().unwrap().send("".as_bytes()).await; // todo: don't leave this empty
                
                if let Ok(response) = result {
                    map.insert(opt.name.clone(), response);
                }
            }
        }

        Ok(map)
    }

    async fn finish(&self) {
        if let Some(connection) = self.connection.clone() {
            let result = connection.disconnect().await;

            if let Err(err) = result {
                warn!("Error while disconnecting: {}", err);
            }
        }
    }

    async fn is_valid(&self) -> bool {
        if let Some(connection) = self.connection.clone() {
            return connection.check_alive().await;
        }
        return false;
    }

    fn get_name(&self) -> &str {
        "MainHub"
    }

    fn get_desc(&self) -> &str {
        "MainHub"
    }

    fn get_devices(&self) -> Vec<Device> {
        self.devices.clone()
    }
}