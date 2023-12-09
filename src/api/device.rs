use serde::{Serialize, Deserialize};
use crate::{DEVICES, api::control_options::ControlOptions};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceType {
    #[serde(rename = "readable")]
    Readable,
    #[serde(rename = "commandable")]
    Commandable
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Device {
    pub id: String,
    #[serde(rename = "type")]
    pub dev_type: DeviceType,
    pub name: String,
    pub desc: String,
    pub img: String,
    pub ctrl_opts: Vec<ControlOptions>,
}

pub async fn search_device(uuid: String) -> Device {
    let devices = DEVICES.lock().await;
    devices.get(&uuid).unwrap().clone()
}