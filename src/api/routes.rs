use std::{collections::HashMap, path::PathBuf};

use actix_web::{Responder, get, HttpResponse, post, web::Json, HttpRequest};
use serde_json::json;

use crate::{DEVICES, HOSTNAME, api::{device::{search_device, access_hub}, verifier::CookieToken}};

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
    match access_hub(device, &req.data).await {
        Ok(_) => HttpResponse::Ok().json(json! {
            {
                "status": "ok"
            }
        }),
        Err(e) => HttpResponse::InternalServerError().json(json! {
            {
                "status": "error",
                "error": e.to_string()
            }
        })
    }
}

#[get("/login")]
async fn client_login(cookie: CookieToken, req: HttpRequest) -> impl Responder {
    if !cookie.authorized {
        let frontend_path = PathBuf::from("./build/index.html");
        actix_files::NamedFile::open(frontend_path).unwrap().into_response(&req)
    } else {
        HttpResponse::PermanentRedirect().append_header(("Location", "/")).finish()
    }
}

#[get("/")]
async fn client_home(cookie: CookieToken, req: HttpRequest) -> impl Responder {
    if cookie.authorized {
        let frontend_path = PathBuf::from("./build/index.html");
        actix_files::NamedFile::open(frontend_path).unwrap().into_response(&req)
    } else {
        HttpResponse::PermanentRedirect().append_header(("Location", "/login")).finish()
    }
}

// logout system
#[get("/logout")]
async fn client_logout() -> impl Responder {
    // todo: remove cookie, access_key for user
    HttpResponse::PermanentRedirect().append_header(("Location", "/login")).finish()
}