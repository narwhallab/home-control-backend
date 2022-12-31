use std::collections::HashMap;

use dyn_clone::DynClone;
use crate::{DEVICES, api::ControlOptions};

#[async_trait::async_trait]
pub trait Device: Sync + Send + DynClone {
    async fn apply(&self, data: &HashMap<String, String>);

    async fn finish(&self) {}

    fn get_name(&self) -> &str { "<no_name>" }

    fn get_desc(&self) -> &str { "" }
    
    fn get_img(&self) -> &str { "/lightbulb.jpg" }

    fn get_ctrl_opts(&self) -> Vec<ControlOptions> { vec![] }

    fn get_id(&self) -> u32;
}

pub async fn search_device(id: u32) -> Box<dyn Device> {
    let opt = DEVICES.lock().unwrap();
    dyn_clone::clone_box(&**opt.get(&id).unwrap())
}

pub fn load(dev: Box<dyn Device>) {
    DEVICES.lock().unwrap().insert(dev.get_id(), dev);
}