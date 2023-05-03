use std::collections::HashMap;

use dyn_clone::DynClone;
use crate::{DEVICES, api::ControlOptions};

#[async_trait::async_trait]
pub trait Hub: Sync + Send + DynClone {
    async fn apply(&self, target: Device, data: &HashMap<String, String>);

    async fn finish(&self);

    fn get_name(&self) -> &str { "<no_name>" }

    fn get_desc(&self) -> &str { "" }

    fn get_devices(&self) -> Vec<Device>;
}

#[derive(Clone)]
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

pub fn load(hub: Box<dyn Hub>) {
    for device in hub.get_devices() {
        DEVICES.lock().unwrap().insert(device.id.clone(), device);
    }
}