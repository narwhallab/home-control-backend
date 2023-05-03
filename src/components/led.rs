use crate::{device::Device, api::ControlOptions};

pub fn new_led_device() -> Device {
    Device {
        id: "287a47cc-f0fa-4575-948a-ffec1e1c7c7b".to_string(),
        name: "LED".to_string(),
        desc: "방 불을 켜거나 끕니다".to_string(),
        img: "/lightbulb.png".to_string(),
        ctrl_opts: vec![
            ControlOptions::new_picker("status", vec!["on", "off"])
        ]
    }
}