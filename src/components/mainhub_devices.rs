use crate::api::{device::{Device, DeviceType}, copts::ControlOptions};

pub fn new_led_device() -> Device {
    Device {
        id: "287a47cc-f0fa-4575-948a-ffec1e1c7c7b".to_string(),
        dev_type: DeviceType::Commandable,
        name: "LED".to_string(),
        desc: "방 불을 켜거나 끕니다".to_string(),
        img: "/lightbulb.png".to_string(),
        ctrl_opts: vec![
            ControlOptions::new_picker("status", vec!["on", "off"])
        ]
    }
}

pub fn new_dist_checker() -> Device {
    Device {
        id: "718b88cf-5df5-418f-aa19-3815cfcdde05".to_string(),
        dev_type: DeviceType::Readable,
        name: "Dist Checker".to_string(),
        desc: "거리를 확인합니다".to_string(),
        img: "/lightbulb.png".to_string(),
        ctrl_opts: vec![
            ControlOptions::new_read("dist_read")
        ]
    }
}