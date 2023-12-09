use std::{collections::HashMap, io::{BufWriter, BufReader}, fs::File};

use home_control_backend::api::{dynamic::DynamicDevice, device::{Device, DeviceType}, control_options::ControlOptions};

fn sample_device() -> DynamicDevice {
    DynamicDevice {
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
    }
}

fn load_file(filename: &str) -> DynamicDevice {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let dynamic_device = DynamicDevice::load_device(reader);

    return dynamic_device;
}

#[test]
fn load_dynamic_device() {
    let dynamic_device = load_file("dynamic_led.json");
    
    assert_eq!(sample_device(), dynamic_device);
}

#[test]
fn save_dynamic_device() {
    let file = File::create("dynamic_led_write.json").unwrap();
    let writer = BufWriter::new(file);
    
    let device = sample_device();
    device.save_device(writer);

    let read_device = load_file("dynamic_led_write.json");

    assert_eq!(read_device, sample_device());

}