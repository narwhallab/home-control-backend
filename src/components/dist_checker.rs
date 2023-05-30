use actix_web::{Responder, get};
use crate::api::{device::{Device, DeviceType, read_hub}, verifier::AuthToken, copts::ControlOptions};

pub fn new_dist_checker() -> Device {
    Device {
        id: "718b88cf-5df5-418f-aa19-3815cfcdde05".to_string(),
        dev_type: DeviceType::READABLE,
        name: "Dist Checker".to_string(),
        desc: "거리를 확인합니다".to_string(),
        img: "/lightbulb.png".to_string(),
        ctrl_opts: vec![
            ControlOptions::new_read("dist_read")
        ]
    }
}

#[get("/dist_checker/test")]
pub async fn test(_auth: AuthToken) -> impl Responder {
    let device = new_dist_checker();
    let data = read_hub(device).await.unwrap();
    data
}