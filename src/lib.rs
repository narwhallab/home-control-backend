pub mod api;
pub mod components;

use std::collections::HashMap;

use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::api::device::{Hub, Device};


const PASSWORD: &str = dotenv!("HASHED_PASSWORD");
const HOSTNAME: &str = "Dolphin2410's Home PC";

lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
    static ref HUBS: Mutex<Vec<Box<dyn Hub>>> = Mutex::new(vec![]);
}