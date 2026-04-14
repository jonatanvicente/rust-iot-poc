// crates/common/src/lib.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorData {
    pub device_id: String,
    pub value: f32,
    pub timestamp: u64,
}