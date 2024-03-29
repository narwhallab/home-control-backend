use std::{collections::HashMap, path::PathBuf};

use actix_web::{Responder, get, HttpResponse, post, web::Json, HttpRequest};
use serde::Deserialize;
use serde_json::json;

use crate::{DEVICES, HOSTNAME, api::{device::{search_device, DeviceType}, hub::{access_hub, read_hub}, control_options::validate_control_data}};

use super::verifier::{AuthToken, CookieToken};

#[get("/api/list_devices")]
async fn list_devices() -> impl Responder {
    HttpResponse::Ok().json(DEVICES.lock().await.clone())
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

#[derive(Deserialize)]
pub struct DeviceControlRequest {
    pub device_id: String,
    pub data: HashMap<String, String>
}

#[post("/api/control_device")]
async fn control_device(_auth: AuthToken, req: Json<DeviceControlRequest>) -> impl Responder {
    let device = search_device(req.device_id.clone()).await;
    if device.dev_type != DeviceType::Commandable {
        return HttpResponse::BadRequest().json(json! {
            {
                "status": "error",
                "error": "device is not commandable"
            }
        })
    }

    if let Ok(_) = validate_control_data(device.ctrl_opts.clone(), &req.data) {
        if let Ok(_) = access_hub(device, &req.data).await {
            return HttpResponse::Ok().json(json! {
                {
                    "status": "ok"
                }
            })
        }
    }

    return HttpResponse::InternalServerError().json(json! {
        {
            "status": "error"
        }
    })
}

#[derive(Deserialize)]
pub struct DeviceFetchInfoRequest {
    pub device_id: String
}

#[post("/api/fetch_info")]
async fn fetch_info(_auth: AuthToken, req: Json<DeviceFetchInfoRequest>) -> impl Responder {
    let device = search_device(req.device_id.clone()).await;
    if device.dev_type != DeviceType::Readable {
        return HttpResponse::BadRequest().json(json! {
            {
                "status": "error",
                "error": "device is not readable"
            }
        })
    }

    match read_hub(device).await {
        Ok(data) => HttpResponse::Ok().json(json! {
            {
                "status": "ok",
                "data": data
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

#[get("/devices")]
async fn devices(cookie: CookieToken, req: HttpRequest) -> impl Responder {
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