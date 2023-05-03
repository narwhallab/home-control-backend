use std::collections::HashMap;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlOptions {
    pub name: String,
    #[serde(rename = "type")]
    pub opt_type: String,
    pub values: Vec<String>
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

// ! changed DeviceData !! Check the frontend part