pub mod device;
pub mod api;

mod routes;
mod components;

use std::{collections::HashMap, sync::Mutex};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, cookie::Key};
use api::ControlOptions;
use device::load;
use routes::configure;
use lazy_static::lazy_static;
use crate::device::Device;

const PASSWORD: &str = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<u32, Box<dyn Device>>> = Mutex::new(HashMap::new());
}

/// Add your devices here
async fn load_devices() {
    load(Box::new(components::LEDDevice::new().await));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session_key = Key::generate();
    load_devices().await;

    HttpServer::new(move || {
        App::new()
            .configure(configure)
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone()).cookie_secure(false).build())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}