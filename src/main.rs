mod api;
mod components;

use std::{collections::HashMap, sync::Mutex};
use actix_web::{App, HttpServer, middleware::Logger};
use api::{device::{load, Hub, Device, DeviceType}, verifier, routes, dynamic::DynamicDevice, copts::ControlOptions};
use components::mainhub::MainHub;
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;

const PASSWORD: &str = dotenv!("HASHED_PASSWORD");
const HOSTNAME: &str = "Dolphin2410's Home PC";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
    static ref HUBS: Mutex<Vec<Box<dyn Hub>>> = Mutex::new(vec![]);
}
/// Add your devices here
async fn load_hubs() {
    load(Box::new(MainHub::new().await));
    // todo: add_dynamic();
}

fn add_dynamic() {
    let dyn_device = DynamicDevice {
        device: Device { 
            id: String::from("7b80a948-37a2-48c9-b2f8-417257a786de"), 
            dev_type: DeviceType::COMMANDABLE, 
            name: String::from("Dynamic Device"), 
            desc: String::from("Dynamic Device"), 
            img: String::from("image"), 
            ctrl_opts: vec![
                ControlOptions::new_range("power", (0.0, 100.0))
            ] },
        bluetooth: String::from(""), // TODO add bluetooth id for test
        handlers: HashMap::from([(String::from("power"), String::from("led:{{{power}}}"))])
    };

    let hub = dyn_device.generate_hub();

    load(Box::new(hub))
    
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
            .service(components::dist_checker::test)
            .service(actix_files::Files::new("/", "./build"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}