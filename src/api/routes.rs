use std::collections::HashMap;

use actix_web::{Responder, get, HttpResponse, post, web::Json};
use serde_json::json;

use crate::{DEVICES, HOSTNAME, api::device::{search_device, access_hub}};

use super::verifier::AuthToken;

#[get("/api/list_devices")]
async fn list_devices() -> impl Responder {
    HttpResponse::Ok().json(DEVICES.lock().unwrap().clone())
}

#[get("/api/server_info")]
async fn server_info() -> impl Responder {
    HttpResponse::Ok().json(json! {
        {
            "hostname": HOSTNAME,
            "version": env!("CARGO_PKG_VERSION"),
        }
    })
}

#[derive(serde::Deserialize)]
pub struct DeviceControlRequest {
    pub device_id: String,
    pub data: HashMap<String, String>
}

#[post("/api/control_device")]
async fn control_device(_auth: AuthToken, req: Json<DeviceControlRequest>) -> impl Responder {
    let device = search_device(req.device_id.clone()).await;
    access_hub(device, &req.data).await.unwrap();

    HttpResponse::Ok().json(json! {
        {
            "status": "ok"
        }
    })
}