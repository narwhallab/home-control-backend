use std::{collections::HashMap, error::Error};

use dyn_clone::DynClone;
use serde::{Serialize, Deserialize};
use crate::{DEVICES, api::copts::ControlOptions, HUBS};

#[async_trait::async_trait]
pub trait Hub: Sync + Send + DynClone {
    async fn apply(&self, target: Device, data: &HashMap<String, String>);

    async fn finish(&self);

    fn get_name(&self) -> &str { "<no_name>" }

    fn get_desc(&self) -> &str { "" }

    fn get_devices(&self) -> Vec<Device>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub img: String,
    pub ctrl_opts: Vec<ControlOptions>,
}

pub async fn search_device(uuid: String) -> Device {
    let devices = DEVICES.lock().unwrap();
    devices.get(&uuid).unwrap().clone()
}

pub(crate) async fn access_hub(device: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let hubs = HUBS.lock().unwrap();
    for hub in hubs.iter() {
        for hub_device in hub.get_devices() {
            if &hub_device.id == &device.id {
                hub.apply(device.clone(), data).await;
                return Ok(())
            }
        }
    }

    Err("Couldn't find device".into())
}

pub(crate) fn load(hub: Box<dyn Hub>) {
    for device in &hub.get_devices() {
        DEVICES.lock().unwrap().insert(device.id.clone(), device.clone());
    }
    HUBS.lock().unwrap().push(hub);
}