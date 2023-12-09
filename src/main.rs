use std::collections::HashMap;
use actix_web::{App, HttpServer, middleware::Logger};
use home_control_backend::api::{device::{load_hub_and_devices, Device, DeviceType}, dynamic::DynamicDevice, control_options::ControlOptions};
use home_control_backend::components::mainhub::MainHub;
use home_control_backend::web::{routes, verifier};

async fn load_hubs() {
    load_hub_and_devices(Box::new(MainHub::new().await)).await; // todo: solve multiple hubs pointing to the same peripheral
    dynamic_led_hub().await;
}

async fn dynamic_led_hub() {
    let dyn_device = DynamicDevice {
        device: Device { 
            id: String::from("7b80a948-37a2-48c9-b2f8-417257a786de"), 
            dev_type: DeviceType::Commandable, 
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
            .service(actix_files::Files::new("/", "./build"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}