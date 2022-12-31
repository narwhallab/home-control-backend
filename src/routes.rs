use std::{path::PathBuf, collections::HashMap};

use actix_session::Session;
use actix_web::{HttpRequest, Responder, HttpResponse, get, post, Error, web::{Json, Form, ServiceConfig}};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sha2::{Sha256, Digest};

use crate::{api::{DeviceData, validate_control_data}, DEVICES, device::search_device, PASSWORD};

type Result<T> = std::result::Result<T, Error>;

fn logged_in(session: &Session) -> Result<bool> {
    if let Some(data) = session.get::<bool>("login")? {
        if data {
            return Ok(true);
        }
    } else {
        session.insert("login", false)?;
    }

    Ok(false)
}

#[get("/api/list_devices")]
/// Returns a list of all devices
async fn route_list_devices(session: Session) -> Result<impl Responder> {
    if !logged_in(&session)? {
        return Ok(HttpResponse::Unauthorized().body("Not logged in"));
    }

    let map = DEVICES.lock().unwrap();
    let devices = map
        .iter()
        .map(|(id, device)|{ DeviceData::from_data(id.to_owned(), device) })
        .collect::<Vec<DeviceData>>();

    Ok(HttpResponse::Ok().json(devices))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A request to control a device
pub struct DataControlRequest {
    pub device_id: u32,
    pub data: HashMap<String, String>
}

#[post("/api/control_device")]
/// Control a device
async fn route_control_device(session: Session, request: Json<DataControlRequest>) -> Result<impl Responder> {
    if !logged_in(&session)? {
        return Ok(HttpResponse::Unauthorized().body("Not logged in"));
    }

    let device_id = request.device_id;
    let device = search_device(device_id).await;

    validate_control_data(device.get_ctrl_opts(), &request.data).unwrap();
    device.apply(&request.data).await;

    Ok(HttpResponse::Ok().finish())
}

#[get("/api/server_info")]
async fn route_show_server_info(session: Session) -> Result<impl Responder> {
    if !logged_in(&session)? {
        return Ok(HttpResponse::Unauthorized().body("Not logged in"));
    }

    let hostname = gethostname::gethostname().into_string().unwrap();
    let version = env!("CARGO_PKG_VERSION");
    Ok(HttpResponse::Ok().json(json! { 
        {
            "hostname": hostname,
            "version": format!("Home Control Server v{}", version)
        }
    }))
}

#[derive(Serialize, Deserialize)]
/// Login Request
struct LoginRequest {
    password: String
}

#[post("/login")]
async fn route_backend_login(session: Session, login_request: Form<LoginRequest>) -> Result<impl Responder> {
    let mut hasher = Sha256::new();
    hasher.update(login_request.password.as_str());

    let hashed_user_input = format!("{:x}", hasher.finalize());

    if hashed_user_input == PASSWORD {
        session.insert("login", true).unwrap(); // Authenticate User
    } else {
        return Ok(HttpResponse::Unauthorized().body("Invalid password"));
    }
    
    Ok(HttpResponse::Found().append_header(("Location", "/")).finish())
}

#[get("/login")]
async fn route_frontend_login(session: Session, req: HttpRequest) -> Result<impl Responder> {
    if logged_in(&session)? {
        return Ok(HttpResponse::Found().append_header(("Location", "/")).finish());
    }

    let frontend_path = PathBuf::from("./public/index.html");
    Ok(actix_files::NamedFile::open(frontend_path).unwrap().into_response(&req))
}

#[get("/logout")]
async fn route_logout(session: Session) -> Result<impl Responder> {
    session.purge();
    Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
}

#[get("/")]
async fn route_index(req: HttpRequest, session: Session) -> Result<impl Responder> {
    if !logged_in(&session)? {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let frontend_path = PathBuf::from("./public/index.html");
    Ok(actix_files::NamedFile::open(frontend_path).unwrap().into_response(&req))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(route_list_devices)
        .service(route_control_device)
        .service(route_show_server_info)
        .service(route_backend_login)
        .service(route_frontend_login)
        .service(route_logout)
        .service(route_index)
        .service(actix_files::Files::new("/", "./public"));
} 