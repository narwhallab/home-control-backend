use std::collections::HashMap;
use narwhal_tooth::{scan_bluetooth, connect_bluetooth, bluetooth::BluetoothConnection};
use crate::{Device, ControlOptions};

#[derive(Clone)]
pub struct LEDDevice {
    //connection: BluetoothConnection
}

impl LEDDevice {
    pub async fn connect() -> BluetoothConnection {
        let result = scan_bluetooth(3).await;
        let hmsoft = result.get_by_name("HMSoft").await.expect("Could not find HMSoft device");
        connect_bluetooth(&hmsoft).await
    }

    pub async fn new() -> Self {
        //let connection = Self::connect().await;
        LEDDevice {
          //  connection
        }
    }
}

#[async_trait::async_trait]
impl Device for LEDDevice {
    async fn apply(&self, data: &HashMap<String, String>) {
        if let Some(data) = data.get("status") {
            let connection = Self::connect().await;
            connection.write(data.as_bytes()).await.unwrap();
        }
    }

    async fn finish(&self) {
      //  self.connection.disconnect().await;
    }

    fn get_name(&self) -> &str {
        "LED"
    }

    fn get_desc(&self) -> &str {
        "방 불을 켜거나 끕니다"
    }

    fn get_img(&self) -> &str {
        "/lightbulb.png"
    }

    fn get_ctrl_opts(&self) -> Vec<ControlOptions> {
        vec![
            ControlOptions::new_picker("status", vec!["on", "off"])
        ]
    }

    fn get_id(&self) -> u32 {
        0
    }
}