pub mod device;
pub mod api;

mod components;
mod verifier;
mod util;

use std::{collections::HashMap, sync::Mutex};
use actix_web::{App, HttpServer, get};
use device::load;
use lazy_static::lazy_static;
use crate::{device::Device, verifier::AuthToken};

const PASSWORD: &str = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
}

/// Add your devices here
async fn load_hubs() {
    load(Box::new(components::MainHub::new().await));
}

#[get("/normal")]
async fn normal() -> &'static str {
    "Normal Request"
}

#[get("/secure")]
async fn secure(_token: AuthToken) -> &'static str {
    "Secure Request"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_hubs().await;

    HttpServer::new(move || {
        App::new()
            .service(verifier::login)
            .service(normal)
            .service(secure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}