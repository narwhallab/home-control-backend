use std::{error::Error, collections::HashMap, fs::File, io::{BufReader, BufWriter}, sync::Mutex};

use narwhal_tooth::{bluetooth::{BluetoothConnection, connect_peripheral}, scan_bluetooth};
use serde::{Serialize, Deserialize};

use super::device::{Device, Hub, DeviceType};

lazy_static::lazy_static! {
    pub static ref EVENT_POOL: Mutex<HashMap<String, Vec<(String, Vec<u8>)>>> = Mutex::new(HashMap::new());
}

// hashmap of bluetooth, vector of (data uuid, data)
pub fn pull() -> HashMap<String, Vec<(String, Vec<u8>)>> {
    let mut buffer = EVENT_POOL.lock().unwrap();
    let cloned = buffer.clone();
    *buffer = HashMap::new();
    return cloned;
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DynamicDevice {
    pub device: Device,
    pub bluetooth: String,
    pub handlers: HashMap<String, String> // source -> raw_command
}

impl DynamicDevice {
    pub fn load_device(target: String) -> DynamicDevice {
        let file = File::open(target).unwrap();
        let read_buf = BufReader::new(file);

        let data: DynamicDevice = serde_json::from_reader(read_buf).unwrap();
        data
    }

    pub fn save_device(&self, target: String) {
        let file = File::create(target).unwrap();
        let write_buf = BufWriter::new(file);

        serde_json::to_writer(write_buf, self).unwrap();
    }

    pub fn generate_hub(&self) -> DynamicHub {
        futures::executor::block_on(async {
            DynamicHub::new(self.clone()).await
        })
    }
}

// TODO: implement
// pub fn register_device(device: Device, bluetooth: String) {

//     todo!()
// }

#[derive(Clone)]
pub struct DynamicHub {
    device: DynamicDevice,
    connection: Option<BluetoothConnection>,
}

impl DynamicHub {
    pub async fn reconnect(device: &DynamicDevice) -> Option<BluetoothConnection> {
        async {
            let scan_results = scan_bluetooth(3).await;
            let hub_device = scan_results.get_by_addr(&device.bluetooth).await.ok_or("Couldn't find bluetooth device")?;
            let opt_conn = connect_peripheral(&hub_device).await;

            if let Ok(conn) = &opt_conn {
                futures::executor::block_on(async {
                    let cloned = device.clone();
                    conn.subscribe(move |v: (String, Vec<u8>)| {
                        EVENT_POOL.lock().unwrap().get_mut(&cloned.bluetooth.clone()).unwrap().push(v);
                    }).await.unwrap();
                });
            }

            opt_conn
        }.await.ok()
    }

    pub async fn new(device: DynamicDevice) -> Self {
        let connection = Self::reconnect(&device).await;

        DynamicHub {
            device,
            connection
        }
    }
}

#[async_trait::async_trait]
impl Hub for DynamicHub {
    async fn apply(&mut self, target: Device, data: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        let connection = if !self.is_valid().await {
            Self::reconnect(&self.device).await
        } else {
            self.connection.clone()
        }.ok_or("Invalid connection")?;

        self.connection = Some(connection.clone());

        if target.dev_type == DeviceType::COMMANDABLE {
            for opt in target.ctrl_opts.iter() {
                if let Some(template) = self.device.handlers.get(&opt.name) {
                    if let Some(data) = data.get(&opt.name) {
                        let command = template.replace(format!("{{{}}}", opt.name).as_str(), data);
                        connection.write(command.as_bytes()).await?;
                    }
                }

            }
        }

        Ok(())
    }

    async fn retreive(&mut self, _target: Device) -> Result<String, Box<dyn Error>> {
        let connection = if !self.is_valid().await {
            Self::reconnect(&self.device).await
        } else {
            self.connection.clone()
        }.ok_or("Invalid connection")?;

        self.connection = Some(connection.clone());

        connection.write("request".as_bytes()).await?;

        // TODO something's weird
        let map = pull();
        if let Some(vec) = map.get(&self.device.bluetooth) {
            if let Some(data) = vec.first() {
                return Ok(String::from_utf8(data.1.clone()).unwrap());
            }
        }
        Ok("".to_string())
    }

    async fn finish(&self) -> Result<(), Box<dyn Error>> {
        if self.connection.is_none() {
            return Ok(());
        }
        self.connection.clone().unwrap().disconnect().await.unwrap();

        Ok(())
    }

    async fn is_valid(&self) -> bool {
        if let Some(value) = self.connection.clone() {
            return value.peripheral_connected().await;
        }
        return false;
    }

    fn get_name(&self) -> &str {
        "MainHub"
    }

    fn get_desc(&self) -> &str {
        "방 불을 켜거나 끕니다"
    }

    fn get_devices(&self) -> Vec<Device> {
        vec![self.device.clone().device]
    }
}