use std::collections::HashMap;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ControllerType {
    #[serde(rename = "picker")]
    Picker,
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "read")]
    Read
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ControlOptions {
    pub name: String,
    #[serde(rename = "type")]
    pub opt_type: ControllerType,
    pub values: Vec<String>,    // if picker
    pub range_min: f32,   // if range
    pub range_max: f32
}

impl ControlOptions {
    pub fn new_picker(name: &str, values: Vec<&str>) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: ControllerType::Picker,
            values: values.iter().map(|s| s.to_string()).collect(),
            range_min: 0.0,
            range_max: 0.0
        }
    }

    pub fn new_range(name: &str, range: (f32, f32)) -> ControlOptions {
        ControlOptions { 
            name: name.to_string(), 
            opt_type: ControllerType::Range, 
            values: vec![], 
            range_min: range.0,
            range_max: range.1
        }
    }

    pub fn new_input(name: &str) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: ControllerType::Input,
            values: vec![],
            range_min: 0.0,
            range_max: 0.0
        }
    }

    pub fn new_read(name: &str) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: ControllerType::Read,
            values: vec![],
            range_min: 0.0,
            range_max: 0.0
        }
    }
}

pub fn validate_control_data(control_options: Vec<ControlOptions>, data: &HashMap<String, String>) -> Result<()> {
    for (key, val) in data.iter() {
        let opt = control_options.iter().find(|opt| opt.name == *key).unwrap();
        match opt.opt_type {
            ControllerType::Picker => {
                if !opt.values.contains(val) {
                    return Err(anyhow::anyhow!("Invalid data"));
                }
            },
            ControllerType::Range => {
                let res = val.parse::<f32>();
                if res.is_err() {
                    return Err(anyhow::anyhow!("Invalid Type: required f32"))
                }
    
                let res = res.unwrap();
    
                if res < opt.range_min || res > opt.range_max { // out of the given range
                    return Err(anyhow::anyhow!("Invalid value: out of range"))
                }
            },
            _ => {}
        }
    }

    return Ok(())
}