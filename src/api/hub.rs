use std::{collections::HashMap, error::Error};

use dyn_clone::DynClone;

use crate::{HUBS, DEVICES};

use super::device::Device;

#[async_trait::async_trait]
pub trait Hub: Sync + Send + DynClone {
    async fn apply(&mut self, target: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>>;

    async fn retreive(&mut self, target: Device) -> Result<HashMap<String, String>, Box<dyn Error>>;

    async fn finish(&self);

    async fn is_valid(&self) -> bool;

    fn get_name(&self) -> &str { "<no_name>" }

    fn get_desc(&self) -> &str { "" }

    fn get_devices(&self) -> Vec<Device>;

    async fn load_devices(&self) {
        for device in self.get_devices() {
            DEVICES.lock().await.insert(device.id.clone(), device.clone());
        }
    }
}

pub(crate) async fn access_hub(device: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut hubs = HUBS.lock().await;
    for hub in hubs.iter_mut() {
        for hub_device in hub.get_devices() {
            if &hub_device.id == &device.id {
                hub.apply(device.clone(), data).await?;
                return Ok(())
            }
        }
    }

    Err("Couldn't find device".into())
}

pub(crate) async fn read_hub(device: Device) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut hubs = HUBS.lock().await;
    for hub in hubs.iter_mut() {
        for hub_device in hub.get_devices() {
            if &hub_device.id == &device.id {
                let data = hub.retreive(device.clone()).await?;
                return Ok(data)
            }
        }
    }

    Err("Couldn't find device".into())
}

pub async fn load_hub_and_devices(hub: Box<dyn Hub>) {
    hub.load_devices().await;
    HUBS.lock().await.push(hub);
}