use std::collections::HashMap;
use actix_web::{Responder, get};
use crate::api::{device::{Device, access_hub}, copts::ControlOptions, verifier::AuthToken};

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

#[get("/led/test")]
pub async fn test(_auth: AuthToken) -> impl Responder {
    let device = new_led_device();
    let mut data = HashMap::new();
    data.insert("status".to_string(), "on".to_string());
    access_hub(device, &data).await.unwrap();
    "done"
}