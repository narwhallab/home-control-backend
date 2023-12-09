use std::{error::Error, collections::HashMap, io::{BufReader, BufWriter, Write, Read}, time::Duration};

use log::warn;
use narwhal_tooth::{bluetooth::BluetoothConnection, scan::scan_bluetooth, util::connect_device};
use serde::{Serialize, Deserialize};

use super::{device::{Device, Hub, DeviceType}, control_options::ControllerType};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DynamicDevice {
    pub device: Device,
    pub bluetooth: String,
    pub handlers: HashMap<String, String> // source -> raw_command
}

impl DynamicDevice {
    pub fn load_device<T>(target: BufReader<T>) -> DynamicDevice where T: Read {
        return serde_json::from_reader(target).unwrap()
    }

    pub fn save_device<T>(&self, target: BufWriter<T>) where T: Write {
        serde_json::to_writer_pretty(target, self).unwrap();
    }

    pub async fn generate_hub(&self) -> DynamicHub {
        DynamicHub::new(self.clone()).await
    }
}

#[derive(Clone)]
pub struct DynamicHub {
    device: DynamicDevice,
    connection: Option<BluetoothConnection>,
}

impl DynamicHub {
    pub async fn reconnect(device: &DynamicDevice) -> Option<BluetoothConnection> {
        {
            let scan_results = scan_bluetooth(Duration::from_secs(3)).await;
            let hub_device = scan_results.search_by_addr(device.bluetooth.clone()).await?;
            let connection_result = connect_device(hub_device.clone()).await;

            connection_result
        }.ok()
    }

    pub async fn new(device: DynamicDevice) -> Self {
        let connection = Self::reconnect(&device).await;

        DynamicHub {
            device,
            connection
        }
    }
}

#[async_trait::async_trait]
impl Hub for DynamicHub {
    async fn apply(&mut self, target: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        if !self.is_valid().await {
            return Err("Device isn't connected".into());
        }

        if target.dev_type != DeviceType::Commandable {
            return Err("Only Commandable Devices can run this function".into());
        }

        for opt in target.ctrl_opts.iter() {
            if let Some(template) = self.device.handlers.get(&opt.name) {
                if let Some(data) = data.get(&opt.name) {
                    let command = template.replace(format!("{{{}}}", opt.name).as_str(), data);
                    let _result = self.connection.clone().unwrap().send(command.as_bytes()).await?; // todo: maybe the result will be useful??
                }
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
        "방 불을 켜거나 끕니다"
    }

    fn get_devices(&self) -> Vec<Device> {
        vec![self.device.clone().device]
    }
}