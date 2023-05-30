use std::collections::HashMap;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlOptions {
    name: String,
    #[serde(rename = "type")]
    opt_type: String,
    values: Vec<String>,    // if picker
    range_min: f32,   // if range
    range_max: f32
}

impl ControlOptions {
    pub fn new_picker(name: &str, values: Vec<&str>) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: "picker".to_string(),
            values: values.iter().map(|s| s.to_string()).collect(),
            range_min: 0.0,
            range_max: 0.0
        }
    }

    pub fn new_range(name: &str, range: (f32, f32)) -> ControlOptions {
        ControlOptions { 
            name: name.to_string(), 
            opt_type: "range".to_string(), 
            values: vec![], 
            range_min: range.0,
            range_max: range.1
        }
    }

    pub fn new_read(name: &str) -> ControlOptions {
        ControlOptions {
            name: name.to_string(),
            opt_type: "read".to_string(),
            values: vec![],
            range_min: 0.0,
            range_max: 0.0
        }
    }
}

pub fn validate_control_data(control_options: Vec<ControlOptions>, data: &HashMap<String, String>) -> Result<()> {
    for (key, val) in data.iter() {
        let opt = control_options.iter().find(|opt| opt.name == *key).unwrap();
        if opt.opt_type == "picker" {       // this type is picker
            if !opt.values.contains(val) {
                return Err(anyhow::anyhow!("Invalid data"));
            }
        } else if opt.opt_type == "range" { // this type is range
            let res = val.parse::<f32>();
            if res.is_err() {
                return Err(anyhow::anyhow!("Invalid Type: required f32"))
            }

            let res = res.unwrap();

            if res < opt.range_min || res > opt.range_max { // out of the given range
                return Err(anyhow::anyhow!("Invalid value: out of range"))
            }
        }
    }

    return Ok(())
}