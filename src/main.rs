mod api;
mod components;

use std::collections::HashMap;
use actix_web::{App, HttpServer, middleware::Logger};
use api::{device::{load_hub_and_devices, Hub, Device, DeviceType}, verifier, routes, dynamic::DynamicDevice, copts::ControlOptions};
use components::mainhub::MainHub;
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

const PASSWORD: &str = dotenv!("HASHED_PASSWORD");
const HOSTNAME: &str = "Dolphin2410's Home PC";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
    static ref HUBS: Mutex<Vec<Box<dyn Hub>>> = Mutex::new(vec![]);
}

async fn load_hubs() {
    load_hub_and_devices(Box::new(MainHub::new().await)).await; // todo: solve multiple hubs pointing to the same peripheral
    dynamic_led_hub().await;
}

async fn dynamic_led_hub() {
    let dyn_device = DynamicDevice {
        device: Device { 
            id: String::from("7b80a948-37a2-48c9-b2f8-417257a786de"), 
            dev_type: DeviceType::COMMANDABLE, 
            name: String::from("[Dyn] LED"), 
            desc: String::from("빛!"), 
            img: String::from("lightbulb.png"), 
            ctrl_opts: vec![
                ControlOptions::new_picker("power", vec!["on", "off"])
            ] },
        bluetooth: String::from("50:33:8B:2A:8D:3C"),
        handlers: HashMap::from([(String::from("power"), String::from("led:{power}"))])
    };

    let dyn_hub = dyn_device.generate_hub().await;

    load_hub_and_devices(Box::new(dyn_hub)).await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_hubs().await;

    println!("Server Started");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(verifier::login)
            .service(routes::list_devices)
            .service(routes::server_info)
            .service(routes::control_device)
            .service(routes::fetch_info)
            .service(routes::client_login)
            .service(routes::client_home)
            .service(routes::devices)
            .service(components::dist_checker::test)
            .service(actix_files::Files::new("/", "./build"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}