use std::collections::HashMap;
use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::device::Device;

#[derive(Serialize, Deserialize)]
pub struct ControlOptions {
    name: String,
    #[serde(rename = "type")]
    opt_type: String,
    values: Vec<String>
}

impl ControlOptions {
    pub fn new_picker(name: &str, values: Vec<&str>) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: "picker".to_string(),
            values: values.iter().map(|s| s.to_string()).collect()
        }
    }
}

pub fn validate_control_data(control_options: Vec<ControlOptions>, data: &HashMap<String, String>) -> Result<()> {
    for (key, val) in data.iter() {
        let opt = control_options.iter().find(|opt| opt.name == *key).unwrap();
        if !opt.values.contains(val) {
            return Err(anyhow::anyhow!("Invalid data"));
        }
    }

    return Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Data of the current server
pub struct DeviceData {
    id: u32,
    title: String,
    desc: String,
    img: String,
    control_options: Vec<ControlOptions>
}

impl DeviceData {
    pub fn from_data(id: u32, device: &Box<dyn Device>) -> DeviceData {
        DeviceData { 
            id,
            title: device.get_name().to_string(),
            desc: device.get_desc().to_string(),
            img: device.get_img().to_string(),
            control_options: device.get_ctrl_opts()
        }
    }
}