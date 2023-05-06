mod api;
mod components;

use std::{collections::HashMap, sync::Mutex};
use actix_web::{App, HttpServer};
use api::{device::{load, Hub, Device}, verifier, routes};
use components::mainhub::MainHub;
use lazy_static::lazy_static;

const PASSWORD: &str = "ca6d31de4acdab8e2ce8aadaeeb44454ba0dcf2d5c188d6d531641bea1f987ae";
const HOSTNAME: &str = "Dolphin2410's Home PC";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
    static ref HUBS: Mutex<Vec<Box<dyn Hub>>> = Mutex::new(vec![]);
}

/// Add your devices here
async fn load_hubs() {
    load(Box::new(MainHub::new().await));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_hubs().await;

    HttpServer::new(move || {
        App::new()
            .service(verifier::login)
            .service(routes::list_devices)
            .service(routes::server_info)
            .service(routes::control_device)
            .service(routes::client_login)
            .service(routes::client_home)
            .service(components::led::test)
            .service(actix_files::Files::new("/", "./build"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}